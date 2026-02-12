use crate::config::board;
use crate::config::color::ColorConfig;
use crate::config::preview::PreviewConfig;
use crate::json::model::*;
use crate::json::predefined::{Custom, Predefined};
use crate::{
    validation, AreaConfig, AreaValueFormatter, BoardConfig, ProgressionConfig,
    PuzzleConfig, PuzzleConfigCollection, PuzzleDifficultyConfig, ReadError, TargetTemplate,
    TileConfig,
};
use ndarray::Array2;
use time::OffsetDateTime;

/// Trait for converting JSON model types to config types.
pub trait Convertable<R> {
    /// Convert the JSON model type to the config type.
    ///
    /// # Arguments
    ///
    /// * `predefined`: Predefined tiles and boards
    /// * `custom`: Instance to store custom tiles and boards. Should initially be empty.
    ///
    /// returns: Result<R, ReadError>
    fn convert(self, predefined: &Predefined, custom: &mut Custom) -> Result<R, ReadError>;
}

impl Convertable<PuzzleConfigCollection> for PuzzleCollection {
    fn convert(
        self,
        predefined: &Predefined,
        custom: &mut Custom,
    ) -> Result<PuzzleConfigCollection, ReadError> {
        if let Some(tiles) = self.custom_tiles {
            for (name, tile) in tiles {
                custom.add_tile(name.clone(), tile);
            }
        }

        if let Some(boards) = self.custom_boards {
            for (name, board) in boards {
                custom.add_board(name.clone(), board);
            }
        }

        let mut puzzle_configs = Vec::new();
        for (i, puzzle) in self.puzzles.into_iter().enumerate() {
            let difficulty_config = puzzle.difficulty.convert(predefined, custom)?;

            let mut tiles = Vec::new();
            for tile_width_index in puzzle.tiles.into_iter().enumerate() {
                let converted_tile = tile_width_index.convert(&predefined, custom)?;
                tiles.push(converted_tile);
            }

            let mut board_config = puzzle.board.convert(&predefined, custom)?;
            if self.allow_board_rotation {
                board_config = rotate_board(board_config);
            }
            let puzzle_config = PuzzleConfig::new(
                i,
                puzzle.id.unwrap_or_else(|| format!("{i}")),
                puzzle.name,
                puzzle.description,
                difficulty_config,
                tiles,
                board_config,
                puzzle.additional_info,
            );
            puzzle_configs.push(puzzle_config);
        }

        Ok(PuzzleConfigCollection::new(
            self.name,
            self.description,
            self.author,
            validation::validate_collection_id(self.id)?,
            self.version,
            self.progression.convert(predefined, custom)?,
            self.preview.convert(predefined, custom)?,
            puzzle_configs,
        ))
    }
}

fn rotate_board_to_landscape<T>(arr: Array2<T>) -> Array2<T> {
    let shape = arr.shape();
    if shape.len() == 2 {
        let height = shape[0];
        let width = shape[1];
        if height < width {
            arr.reversed_axes()
        } else {
            arr
        }
    } else {
        arr
    }
}

fn rotate_board(board: BoardConfig) -> BoardConfig {
    match board {
        BoardConfig::Simple { layout } => {
            let layout = rotate_board_to_landscape(layout);
            BoardConfig::Simple { layout }
        }
        BoardConfig::Area {
            layout,
            area_indices,
            display_values,
            value_order,
            area_configs,
            target_template,
        } => {
            let layout = rotate_board_to_landscape(layout);
            let area_indices = rotate_board_to_landscape(area_indices);
            let display_values = rotate_board_to_landscape(display_values);
            let value_order = rotate_board_to_landscape(value_order);
            BoardConfig::Area {
                layout,
                area_indices,
                display_values,
                value_order,
                area_configs,
                target_template,
            }
        }
    }
}

impl Convertable<Option<PuzzleDifficultyConfig>> for Option<PuzzleDifficulty> {
    fn convert(
        self,
        _: &Predefined,
        _: &mut Custom,
    ) -> Result<Option<PuzzleDifficultyConfig>, ReadError> {
        match self {
            Some(PuzzleDifficulty::Easy) => Ok(Some(PuzzleDifficultyConfig::Easy)),
            Some(PuzzleDifficulty::Medium) => Ok(Some(PuzzleDifficultyConfig::Medium)),
            Some(PuzzleDifficulty::Hard) => Ok(Some(PuzzleDifficultyConfig::Hard)),
            Some(PuzzleDifficulty::Expert) => Ok(Some(PuzzleDifficultyConfig::Expert)),
            None => Ok(None),
        }
    }
}

impl Convertable<ProgressionConfig> for Progression {
    fn convert(self, _: &Predefined, _: &mut Custom) -> Result<ProgressionConfig, ReadError> {
        match self {
            Progression::Any => Ok(ProgressionConfig::Any),
            Progression::Sequential => Ok(ProgressionConfig::Sequential),
        }
    }
}

impl Convertable<PreviewConfig> for Option<Preview> {
    fn convert(self, _: &Predefined, _: &mut Custom) -> Result<PreviewConfig, ReadError> {
        match self {
            None => Ok(PreviewConfig::default()),
            Some(preview) => Ok(PreviewConfig::new(
                preview.show_board,
                preview.show_board_size,
                preview.show_tiles,
                preview.show_tile_count,
            )),
        }
    }
}

impl Convertable<TileConfig> for (usize, Tile) {
    fn convert(
        self,
        predefined: &Predefined,
        custom: &mut Custom,
    ) -> Result<TileConfig, ReadError> {
        match self.1 {
            Tile::Ref(name) => {
                if let Some(predefined_tile) = predefined.get_tile(&name) {
                    (self.0, predefined_tile).convert(predefined, custom)
                } else if let Some(custom_tile) = custom.get_tile(&name) {
                    (self.0, custom_tile).convert(predefined, custom)
                } else {
                    Err(ReadError::UnknownPredefinedTile { name })
                }
            }
            Tile::Layout(layout) => {
                let base = (self.0, layout).convert(predefined, custom)?;
                let color = (self.0, None).convert(predefined, custom)?;
                Ok(TileConfig::new(base, color))
            }
            Tile::Custom { layout, color } => {
                let base = (self.0, layout).convert(predefined, custom)?;
                let color = (self.0, color).convert(predefined, custom)?;
                Ok(TileConfig::new(base, color))
            }
        }
    }
}

impl Convertable<Array2<bool>> for (usize, TileLayout) {
    fn convert(
        self,
        predefined: &Predefined,
        custom: &mut Custom,
    ) -> Result<Array2<bool>, ReadError> {
        match self.1 {
            TileLayout::Ref(name) => {
                if let Some(custom_tile) = custom.get_tile(&name) {
                    Ok((self.0, custom_tile)
                        .convert(predefined, custom)?
                        .base()
                        .clone())
                } else if let Some(predefined_tile) = predefined.get_tile(&name) {
                    Ok((self.0, predefined_tile)
                        .convert(predefined, custom)?
                        .base()
                        .clone())
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
                let mut base = Array2::<bool>::default((height, width));
                for (i, row) in array.iter().enumerate() {
                    for (j, &value) in row.iter().enumerate() {
                        base[(i, j)] = value != 0;
                    }
                }
                let base = base.reversed_axes();
                Ok(base)
            }
        }
    }
}

impl Convertable<ColorConfig> for (usize, Option<Color>) {
    fn convert(self, _: &Predefined, _: &mut Custom) -> Result<ColorConfig, ReadError> {
        match self.1 {
            None => Ok(ColorConfig::default_with_index(self.0)),
            Some(Color::Hex(hex)) => {
                ColorConfig::try_from(hex).map_err(|e| ReadError::InvalidColor { message: e })
            }
        }
    }
}

impl Convertable<BoardConfig> for Board {
    fn convert(
        self,
        predefined: &Predefined,
        custom: &mut Custom,
    ) -> Result<BoardConfig, ReadError> {
        match { self } {
            Board::Ref(name) => {
                if let Some(custom_board) = custom.get_board(&name) {
                    Ok(custom_board.convert(predefined, custom)?)
                } else if let Some(predefined_board) = predefined.get_board(&name) {
                    Ok(predefined_board.convert(predefined, custom)?)
                } else if let Some(predefined_board) = board::from_predefined_board(&name) {
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
                let mut array = Array2::<bool>::default((height, width));
                for (i, row) in layout.iter().enumerate() {
                    for (j, &value) in row.iter().enumerate() {
                        array[(i, j)] = value < 1;
                    }
                }
                let array = array.reversed_axes();
                Ok(BoardConfig::Simple { layout: array })
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
                    .map(|a| a.convert(predefined, custom))
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
                    let mut array = Array2::<bool>::default((height, width));
                    for (i, row) in area_layout.iter().enumerate() {
                        for (j, &value) in row.iter().enumerate() {
                            array[(i, j)] = value >= 0;
                        }
                    }
                    let array = array.reversed_axes();
                    array
                };

                Ok(BoardConfig::Area {
                    layout: board_layout,
                    area_indices: vec_vec_to_array2(&area_layout).reversed_axes(),
                    display_values: vec_vec_to_array2(&values).reversed_axes(),
                    value_order: vec_vec_to_array2(&value_order).reversed_axes(),
                    area_configs,
                    target_template: TargetTemplate::new(&target_template),
                })
            }
        }
    }
}

fn vec_vec_to_array2<T: Clone + Default>(data: &Vec<Vec<T>>) -> Array2<T> {
    let height = data.len();
    let width = if height > 0 { data[0].len() } else { 0 };
    let mut array = Array2::<T>::default((height, width));
    for (i, row) in data.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            array[(i, j)] = value.clone();
        }
    }
    array
}

impl Convertable<AreaConfig> for Area {
    fn convert(
        self,
        predefined: &Predefined,
        custom: &mut Custom,
    ) -> Result<AreaConfig, ReadError> {
        let formatter = match &self.formatter {
            AreaFormatter::Plain => AreaValueFormatter::Plain,
            AreaFormatter::Nth => AreaValueFormatter::Nth,
            AreaFormatter::PrefixSuffix { prefix, suffix } => AreaValueFormatter::PrefixSuffix {
                prefix: prefix.clone(),
                suffix: suffix.clone(),
            },
        };

        Ok(AreaConfig::new(
            self.name.clone(),
            formatter,
            self.default_factory.convert(predefined, custom)?,
        ))
    }
}

impl Convertable<String> for DefaultFactory {
    fn convert(self, _: &Predefined, _: &mut Custom) -> Result<String, ReadError> {
        match self {
            DefaultFactory::Fixed { value } => Ok(value.to_string()),
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_convert_predefined_tile() {
        let mut predefined = Predefined::default();
        predefined.add_tile(
            "L3".to_string(),
            Tile::Layout(TileLayout::Custom(vec![vec![1, 0], vec![1, 1]])),
        );

        let tile = Tile::Ref("L3".to_string());
        let converted_tile = (0, tile)
            .convert(&predefined, &mut Custom::default())
            .unwrap();
        let expected_tile = TileConfig::new(
            arr2(&[[true, false], [true, true]]).reversed_axes(),
            ColorConfig::default_with_index(0),
        );
        assert_eq!(converted_tile.base(), expected_tile.base());
    }

    #[test]
    fn test_convert_predefined_tile_unknown() {
        let tile = Tile::Ref("test".to_string());
        let converted_tile = (0, tile).convert(&Predefined::default(), &mut Custom::default());
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
        let tile = Tile::Layout(TileLayout::Custom(vec![vec![1, 0], vec![1, 1]]));
        let converted_tile = (0, tile)
            .convert(&Predefined::default(), &mut Custom::default())
            .unwrap();
        let expected_tile = TileConfig::new(
            arr2(&[[true, false], [true, true]]).reversed_axes(),
            ColorConfig::default_with_index(0),
        );
        assert_eq!(converted_tile.base(), expected_tile.base());
    }

    #[test]
    fn test_convert_custom_tile_zero_dimension() {
        let tile = Tile::Layout(TileLayout::Custom(vec![]));
        let converted_tile = (0, tile).convert(&Predefined::default(), &mut Custom::default());
        assert!(converted_tile.is_err());
        assert_eq!(
            converted_tile.err().unwrap(),
            ReadError::TileWidthOrHeightCannotBeZero
        );

        let tile = Tile::Layout(TileLayout::Custom(vec![vec![1, 0], vec![]]));
        let converted_tile = (0, tile).convert(&Predefined::default(), &mut Custom::default());
        assert!(converted_tile.is_err());
        assert_eq!(
            converted_tile.err().unwrap(),
            ReadError::TileWidthOrHeightCannotBeZero
        );
    }
}
