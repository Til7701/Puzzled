use crate::config::board::AreaBoardData;
use crate::json::model::{
    Area, AreaFormatter, Board, Color, DefaultFactory, PolyformKind, Preview, Progression,
    PuzzleCollection, PuzzleDifficulty, Tile, TileLayout,
};
use crate::json::predefined::{Custom, Predefined};
use crate::{
    AreaConfig, AreaValueFormatter, BoardConfig, ColorConfig, PreviewConfig, ProgressionConfig,
    PuzzleConfig, PuzzleConfigCollection, PuzzleDifficultyConfig, ReadError, TargetTemplate,
    TileConfig, validation,
};
use puzzled_polyform::{Poly, Polyform, Polyhex, Polyomino};
use std::hash::Hash;
use std::num::NonZero;
use time::OffsetDateTime;

pub struct Converter<'a> {
    predefined: &'a Predefined,
    custom: Custom,
}

impl<'a> Converter<'a> {
    pub fn new(predefined: &'a Predefined) -> Self {
        Converter {
            predefined,
            custom: Custom::default(),
        }
    }

    pub fn convert_collection(
        &mut self,
        collection_config: PuzzleCollection,
    ) -> Result<PuzzleConfigCollection, ReadError> {
        if let Some(tiles) = collection_config.custom_tiles {
            for (name, tile) in tiles {
                self.custom.add_tile(name, tile);
            }
        }
        if let Some(boards) = collection_config.custom_boards {
            for (name, board) in boards {
                self.custom.add_board(name, board);
            }
        }

        let mut puzzle_configs = Vec::new();
        for (i, puzzle) in collection_config.puzzles.into_iter().enumerate() {
            let polyform_kind = collection_config
                .polyform_kind
                .or(puzzle.polyform_kind)
                .unwrap_or(PolyformKind::Polyomino);
            let difficulty_config = self.convert_puzzle_difficulty(puzzle.difficulty)?;

            let mut tiles = Vec::with_capacity(puzzle.tiles.len());
            let mut index_offset = 0;
            for (tile_index, tile) in puzzle.tiles.into_iter().enumerate() {
                let converted_tile =
                    self.convert_tile(index_offset + tile_index, tile, None, polyform_kind)?;
                index_offset += converted_tile.len() - 1;
                tiles.extend(converted_tile);
            }

            let mut board_config = self.convert_board(puzzle.board, polyform_kind)?;
            if collection_config.allow_board_rotation {
                board_config = Converter::rotate_board(board_config);
            }
            let puzzle_config = PuzzleConfig::new(
                i,
                puzzle.id.unwrap_or_else(|| format!("{i}")),
                puzzle.name,
                puzzle.description,
                difficulty_config,
                puzzle.unsolvable,
                tiles,
                board_config,
                puzzle.additional_info,
            );
            puzzle_configs.push(puzzle_config);
        }

        Ok(PuzzleConfigCollection::new(
            collection_config.name,
            collection_config.description,
            collection_config.author,
            validation::validate_collection_id(collection_config.id)?,
            collection_config.version,
            self.convert_collection_progression(collection_config.progression)?,
            self.convert_preview(collection_config.preview)?,
            puzzle_configs,
        ))
    }

    fn rotate_board(mut board: BoardConfig) -> BoardConfig {
        match &mut board {
            BoardConfig::Simple { layout } => {
                layout.rotate_to_landscape();
            }
            BoardConfig::Area { layout, .. } => {
                layout.rotate_to_landscape();
            }
        };
        board
    }

    fn convert_puzzle_difficulty(
        &mut self,
        difficulty: Option<PuzzleDifficulty>,
    ) -> Result<Option<PuzzleDifficultyConfig>, ReadError> {
        Ok(difficulty.map(|d| match d {
            PuzzleDifficulty::Easy => PuzzleDifficultyConfig::Easy,
            PuzzleDifficulty::Medium => PuzzleDifficultyConfig::Medium,
            PuzzleDifficulty::Hard => PuzzleDifficultyConfig::Hard,
            PuzzleDifficulty::Expert => PuzzleDifficultyConfig::Expert,
        }))
    }

    fn convert_collection_progression(
        &mut self,
        progression: Progression,
    ) -> Result<ProgressionConfig, ReadError> {
        Ok(match progression {
            Progression::Any => ProgressionConfig::Any,
            Progression::Sequential => ProgressionConfig::Sequential,
        })
    }

    fn convert_preview(&mut self, preview: Option<Preview>) -> Result<PreviewConfig, ReadError> {
        match preview {
            None => Ok(PreviewConfig::default()),
            Some(preview) => Ok(PreviewConfig::new(
                preview.show_board,
                preview.show_board_size,
                preview.show_tiles,
                preview.show_tile_count,
            )),
        }
    }

    fn convert_tile(
        &mut self,
        tile_id: usize,
        tile: Tile,
        name: Option<String>,
        polyform_kind: PolyformKind,
    ) -> Result<Vec<TileConfig>, ReadError> {
        match tile {
            Tile::Ref(name) => {
                if let Some(custom_tile) = self.custom.get_tile(&name) {
                    self.convert_tile(tile_id, custom_tile, Some(name), polyform_kind)
                } else if let Some(predefined_tile) = self.predefined.get_tile(&name) {
                    self.convert_tile(tile_id, predefined_tile, Some(name), polyform_kind)
                } else {
                    Err(ReadError::UnknownPredefinedTile { name })
                }
            }
            Tile::Layout(layout) => {
                let (base, layout_name) =
                    self.convert_tile_layout(tile_id, layout, polyform_kind)?;
                let color = self.convert_tile_color(tile_id, None)?;
                Ok(vec![TileConfig::new(base, color, layout_name.or(name))])
            }
            Tile::Custom {
                layout,
                color,
                count,
            } => {
                let (base, name) = self.convert_tile_layout(tile_id, layout, polyform_kind)?;
                let count = count.unwrap_or_else(|| NonZero::new(1).unwrap());

                let mut tiles = Vec::with_capacity(count.get() as usize);
                for i in 0..count.get() {
                    let tile_index = tile_id + i as usize;
                    let color = self.convert_tile_color(tile_index, color.clone())?;
                    tiles.push(TileConfig::new(base.clone(), color, name.clone()));
                }
                Ok(tiles)
            }
        }
    }

    fn convert_tile_layout(
        &mut self,
        tile_id: usize,
        layout: TileLayout,
        polyform_kind: PolyformKind,
    ) -> Result<(Polyform<()>, Option<String>), ReadError> {
        match layout {
            TileLayout::Ref(name) => {
                if let Some(custom_tile) = self.custom.get_tile(&name) {
                    Ok((
                        self.convert_tile(tile_id, custom_tile, Some(name.clone()), polyform_kind)?
                            .first()
                            .unwrap()
                            .base()
                            .clone(),
                        Some(name),
                    ))
                } else if let Some(predefined_tile) = self.predefined.get_tile(&name) {
                    Ok((
                        self.convert_tile(
                            tile_id,
                            predefined_tile,
                            Some(name.clone()),
                            polyform_kind,
                        )?
                        .first()
                        .unwrap()
                        .base()
                        .clone(),
                        Some(name),
                    ))
                } else {
                    Err(ReadError::UnknownPredefinedTile { name })
                }
            }
            TileLayout::Custom(array) => {
                let height = array.len();
                if height == 0 {
                    return Err(ReadError::TileWidthOrHeightCannotBeZero);
                }
                let width = array[0].len();
                for row in &array {
                    if row.len() != width {
                        return Err(ReadError::TileWidthOrHeightCannotBeZero);
                    }
                }
                let mapper = |value: &i8, _| {
                    if *value != 0 { Some(()) } else { None }
                };
                let base = Self::create_polyform(&array, mapper, polyform_kind);

                Ok((base, None))
            }
        }
    }

    fn convert_tile_color(
        &mut self,
        tile_id: usize,
        color: Option<Color>,
    ) -> Result<ColorConfig, ReadError> {
        match color {
            None => Ok(ColorConfig::default_with_index(tile_id)),
            Some(Color::Hex(hex)) => {
                ColorConfig::try_from(hex).map_err(|e| ReadError::InvalidColor { message: e })
            }
        }
    }

    fn convert_board(
        &mut self,
        board: Board,
        polyform_kind: PolyformKind,
    ) -> Result<BoardConfig, ReadError> {
        match board {
            Board::Ref(name) => {
                if let Some(custom_board) = self.custom.get_board(&name) {
                    self.convert_board(custom_board, polyform_kind)
                } else if let Some(predefined_board) = self.predefined.get_board(&name) {
                    self.convert_board(predefined_board, polyform_kind)
                } else if let Some(predefined_board) =
                    self.predefined.predefined_board_from_str(&name)
                {
                    Ok(predefined_board)
                } else {
                    Err(ReadError::UnknownCustomBoard {
                        puzzle_name: "unknown".to_string(),
                        board_name: name,
                    })
                }
            }
            Board::SimpleBoard { layout } => {
                let height = layout.len();
                if height == 0 {
                    return Err(ReadError::BoardWidthOrHeightCannotBeZero);
                }
                let width = layout[0].len();
                for row in &layout {
                    if row.len() != width {
                        return Err(ReadError::BoardWidthOrHeightCannotBeZero);
                    }
                }

                let mapper = |value: &u8, _| {
                    if *value < 1 { Some(()) } else { None }
                };
                let polyform = Self::create_polyform(&layout, mapper, polyform_kind);

                Ok(BoardConfig::Simple { layout: polyform })
            }
            Board::AreaBoard {
                area_layout,
                values,
                value_order,
                areas,
                target_template,
            } => {
                let area_configs = areas
                    .into_iter()
                    .map(|a| self.convert_area(a))
                    .collect::<Result<Vec<AreaConfig>, ReadError>>()?;

                let board_layout = {
                    let height = area_layout.len();
                    if height == 0 {
                        return Err(ReadError::BoardWidthOrHeightCannotBeZero);
                    }
                    let width = area_layout[0].len();
                    for row in &area_layout {
                        if row.len() != width {
                            return Err(ReadError::BoardWidthOrHeightCannotBeZero);
                        }
                    }

                    let mapper = |value: &i32, (x, y): (usize, usize)| -> Option<AreaBoardData> {
                        if *value >= 0 {
                            let area_index = area_layout.get(x).and_then(|v| v.get(y))?;
                            let display_value = values.get(x).and_then(|v| v.get(y))?;
                            let value_order = value_order.get(x).and_then(|v| v.get(y))?;
                            Some(AreaBoardData {
                                area_index: *area_index,
                                display_value: display_value.clone(),
                                value_order: *value_order,
                            })
                        } else {
                            None
                        }
                    };
                    Self::create_polyform(&area_layout, mapper, polyform_kind)
                };

                Ok(BoardConfig::Area {
                    layout: board_layout,
                    area_configs,
                    target_template: TargetTemplate::new(&target_template),
                })
            }
        }
    }

    fn convert_area(&mut self, area: Area) -> Result<AreaConfig, ReadError> {
        let formatter = match area.formatter {
            AreaFormatter::Plain => AreaValueFormatter::Plain,
            AreaFormatter::Nth => AreaValueFormatter::Nth,
            AreaFormatter::PrefixSuffix { prefix, suffix } => {
                AreaValueFormatter::PrefixSuffix { prefix, suffix }
            }
        };

        Ok(AreaConfig::new(
            area.name,
            formatter,
            self.convert_default_factory(area.default_factory)?,
        ))
    }

    fn convert_default_factory(
        &mut self,
        default_factory: DefaultFactory,
    ) -> Result<String, ReadError> {
        match default_factory {
            DefaultFactory::Fixed { value } => Ok(value),
            DefaultFactory::CurrentDay => {
                let date =
                    OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
                Ok(date.day().to_string())
            }
            DefaultFactory::CurrentMonthShort => {
                let date =
                    OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
                let month_str = &date.month().to_string()[0..3];
                Ok(month_str.to_string())
            }
            DefaultFactory::CurrentYear2FirstDigit => {
                let date =
                    OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
                let year = date.year() % 100;
                let first_digit = year / 10;
                Ok(first_digit.to_string())
            }
            DefaultFactory::CurrentYear2SecondDigit => {
                let date =
                    OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
                let year = date.year() % 100;
                let second_digit = year % 10;
                Ok(second_digit.to_string())
            }
            DefaultFactory::CurrentYear4FirstDigit => {
                let date =
                    OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
                let year = date.year();
                let first_digit = year / 1000;
                Ok(first_digit.to_string())
            }
            DefaultFactory::CurrentYear4SecondDigit => {
                let date =
                    OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
                let year = date.year();
                let second_digit = (year % 1000) / 100;
                Ok(second_digit.to_string())
            }
        }
    }

    fn create_polyform<T, M, R>(
        vec: &[Vec<T>],
        mapper: M,
        polyform_kind: PolyformKind,
    ) -> Polyform<R>
    where
        T: Clone + Hash,
        M: Fn(&T, (usize, usize)) -> Option<R>,
        R: Clone + Hash,
    {
        match polyform_kind {
            PolyformKind::Polyomino => {
                let mut polyomino = Polyomino::from_vec(vec, &mapper);
                polyomino.transpose();
                polyomino.into()
            }
            PolyformKind::Polyhex => Polyhex::from_vec(vec, &mapper).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use puzzled_polyform::*;

    #[test]
    fn test_convert_predefined_tile() {
        let mut predefined = Predefined::default();
        predefined.add_tile(
            "L3".to_string(),
            Tile::Layout(TileLayout::Custom(vec![vec![1, 0], vec![1, 1]])),
        );
        let mut converter = Converter::new(&predefined);

        let tile = Tile::Ref("L3".to_string());
        let converted_tile = converter
            .convert_tile(0, tile, Some("L3".to_string()), PolyformKind::Polyomino)
            .unwrap();
        let expected_tile = TileConfig::new(
            {
                let mut p = Polyomino::from_bool_slice(&[[true, false], [true, true]]);
                p.transpose();
                p.into()
            },
            ColorConfig::default_with_index(0),
            Some("L3".to_string()),
        );
        assert_eq!(*converted_tile.first().unwrap(), expected_tile);
    }

    #[test]
    fn test_convert_predefined_tile_unknown() {
        let predefined = Predefined::default();
        let mut converter = Converter::new(&predefined);

        let tile = Tile::Ref("test".to_string());
        let converted_tile =
            converter.convert_tile(0, tile, Some("L3".to_string()), PolyformKind::Polyomino);
        assert!(converted_tile.is_err());
        assert_eq!(
            converted_tile.err().unwrap(),
            ReadError::UnknownPredefinedTile {
                name: "test".to_string()
            }
        );
    }

    #[test]
    fn test_convert_custom_tile() {
        let predefined = Predefined::default();
        let mut converter = Converter::new(&predefined);

        let tile = Tile::Layout(TileLayout::Custom(vec![vec![1, 0], vec![1, 1]]));
        let converted_tile = converter
            .convert_tile(0, tile, Some("L3".to_string()), PolyformKind::Polyomino)
            .unwrap();
        let expected_tile = TileConfig::new(
            {
                let mut p = Polyomino::from_bool_slice(&[[true, false], [true, true]]);
                p.transpose();
                p.into()
            },
            ColorConfig::default_with_index(0),
            Some("L3".to_string()),
        );
        assert_eq!(*converted_tile.first().unwrap(), expected_tile);
    }

    #[test]
    fn test_convert_custom_tile_zero_dimension() {
        let predefined = Predefined::default();
        let mut converter = Converter::new(&predefined);

        let tile = Tile::Layout(TileLayout::Custom(vec![]));
        let converted_tile =
            converter.convert_tile(0, tile, Some("L3".to_string()), PolyformKind::Polyomino);
        assert!(converted_tile.is_err());
        assert_eq!(
            converted_tile.err().unwrap(),
            ReadError::TileWidthOrHeightCannotBeZero
        );

        let tile = Tile::Layout(TileLayout::Custom(vec![vec![1, 0], vec![]]));
        let converted_tile =
            converter.convert_tile(0, tile, Some("L3".to_string()), PolyformKind::Polyomino);
        assert!(converted_tile.is_err());
        assert_eq!(
            converted_tile.err().unwrap(),
            ReadError::TileWidthOrHeightCannotBeZero
        );
    }
}
