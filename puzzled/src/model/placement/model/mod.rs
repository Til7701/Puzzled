use crate::app::puzzle::puzzle_area::puzzle_state::{
    Cell, PuzzleState, TileCellPlacement, UnusedTile,
};
use crate::model::extension::PuzzleTypeExtension;
use crate::model::placement::pixel::PixelPosition;
use crate::model::placement::prelude::{
    FullPlacementModel, InternalPlacementModel, SimplePlacementModel,
};
use crate::model::puzzle::PuzzleModel;
use hexomino::HexominoPlacementModel;
use polyomino::PolyominoPlacementModel;
use puzzled_common::polyform::{CoordBox, MutPrototile, Poly, Polyform, Prototile, RegularCoord};
use std::cell::Ref;

pub mod hexomino;
pub mod polyomino;

#[derive(Debug)]
pub enum PlacementModel {
    Polyomino(PolyominoPlacementModel),
    Hexomino(HexominoPlacementModel),
}

impl Default for PlacementModel {
    fn default() -> Self {
        PlacementModel::Polyomino(PolyominoPlacementModel::default())
    }
}

impl FullPlacementModel<Polyform<()>, CoordBox> for PlacementModel {
    fn find_tile_matching_base(&self, base: &Polyform<()>) -> Option<usize> {
        match (self, base) {
            (PlacementModel::Polyomino(p), Polyform::Polyomino(b)) => p
                .tiles()
                .iter()
                .enumerate()
                .find(|t| t.1.base() == b)
                .map(|t| t.0),
            (PlacementModel::Hexomino(h), Polyform::Hexomino(b)) => h
                .tiles()
                .iter()
                .enumerate()
                .find(|t| t.1.base() == b)
                .map(|t| t.0),
            _ => None,
        }
    }

    fn extract_puzzle_state(
        &self,
        puzzle: &PuzzleModel,
        puzzle_type_extension: Ref<Option<PuzzleTypeExtension>>,
    ) -> Result<PuzzleState, String> {
        let puzzle_config = puzzle.config();

        let mut state = PuzzleState::new(puzzle_config, puzzle_type_extension);

        let board_position = self.board_cell_position();
        let board_size = puzzle_config.board_config().layout().dim();

        match self {
            PlacementModel::Polyomino(p) => {
                for (i, tile) in p.tiles().iter().enumerate() {
                    self.add_tile(
                        &mut state,
                        board_position,
                        board_size,
                        TileToPlace {
                            i,
                            current_rotation: &tile.current_rotation().clone().into(),
                            base: &tile.base().clone().into(),
                            name: tile.name(),
                        },
                    )?;
                }
            }
            PlacementModel::Hexomino(h) => {
                for (i, tile) in h.tiles().iter().enumerate() {
                    self.add_tile(
                        &mut state,
                        board_position,
                        board_size,
                        TileToPlace {
                            i,
                            current_rotation: &tile.current_rotation().clone().into(),
                            base: &tile.base().clone().into(),
                            name: tile.name(),
                        },
                    )?;
                }
            }
        };

        Ok(state)
    }

    fn board_pixel_position(&self) -> PixelPosition {
        match self {
            PlacementModel::Polyomino(p) => p.board().position_pixel(),
            PlacementModel::Hexomino(h) => h.board().position_pixel(),
        }
    }

    fn board_cell_position(&self) -> CoordBox {
        match self {
            PlacementModel::Polyomino(p) => p.board().position_cells().into(),
            PlacementModel::Hexomino(h) => h.board().position_cells().into(),
        }
    }

    fn board_size(&self) -> PixelPosition {
        match self {
            PlacementModel::Polyomino(p) => p.board().pixel_size(),
            PlacementModel::Hexomino(h) => h.board().pixel_size(),
        }
    }

    fn tile_pixel_position(&self, idx: usize) -> Option<PixelPosition> {
        match self {
            PlacementModel::Polyomino(p) => p.tiles().get(idx).map(|t| t.position_pixels()),
            PlacementModel::Hexomino(h) => h.tiles().get(idx).map(|t| t.position_pixels()),
        }
    }

    fn tile_cell_position(&self, idx: usize) -> Option<CoordBox> {
        match self {
            PlacementModel::Polyomino(p) => p.tiles().get(idx).map(|t| t.position_cells().into()),
            PlacementModel::Hexomino(h) => h.tiles().get(idx).map(|t| t.position_cells().into()),
        }
    }

    fn tile_size(&self, idx: usize) -> PixelPosition {
        match self {
            PlacementModel::Polyomino(p) => p.tiles().get(idx).map(|t| t.pixel_size()).unwrap(),
            PlacementModel::Hexomino(h) => h.tiles().get(idx).map(|t| t.pixel_size()).unwrap(),
        }
    }

    fn hint_tile_position(&self) -> Option<PixelPosition> {
        match self {
            PlacementModel::Polyomino(p) => p.hint_tile().as_ref().map(|t| t.position_pixels()),
            PlacementModel::Hexomino(h) => h.hint_tile().as_ref().map(|t| t.position_pixels()),
        }
    }

    fn hint_tile_size(&self) -> Option<PixelPosition> {
        match self {
            PlacementModel::Polyomino(p) => p.hint_tile().as_ref().map(|t| t.pixel_size()),
            PlacementModel::Hexomino(h) => h.hint_tile().as_ref().map(|t| t.pixel_size()),
        }
    }
}

struct TileToPlace<'a> {
    i: usize,
    current_rotation: &'a Polyform<()>,
    base: &'a Polyform<()>,
    name: &'a Option<String>,
}

impl PlacementModel {
    fn add_tile(
        &self,
        state: &mut PuzzleState,
        board_position: CoordBox,
        board_size: CoordBox,
        tile_to_place: TileToPlace,
    ) -> Result<(), String> {
        let i = tile_to_place.i;
        let tile_current_rotation = tile_to_place.current_rotation;
        let tile_base = tile_to_place.base;
        let tile_name = tile_to_place.name;

        let tile_position = self
            .tile_cell_position(i)
            .ok_or_else(|| "Tile position not set".to_string())?;
        let one: CoordBox = match tile_position {
            CoordBox::Regular(_) => RegularCoord::new(1, 1).into(),
            CoordBox::Hex(_) => todo!(),
        };
        let mut any_cell_on_board = false;
        for prototile in tile_current_rotation.iter() {
            let cell_position = tile_position + prototile.coord();
            if &cell_position >= &(board_position - one)
                && &cell_position < &(board_position + board_size)
            {
                let cell_position_on_board = cell_position - board_position + one;
                match state.grid.get_mut(cell_position_on_board) {
                    None => {}
                    Some(mut cell_ref) => {
                        let tile_cell_placement = TileCellPlacement {
                            tile_id: i,
                            cell_position: prototile.coord(),
                        };
                        let new = match cell_ref.data().clone() {
                            Cell::Empty(data) => {
                                any_cell_on_board = any_cell_on_board || data.is_on_board;
                                Cell::One(data, tile_cell_placement)
                            }
                            Cell::One(data, existing_widget) => {
                                any_cell_on_board = any_cell_on_board || data.is_on_board;
                                let widgets = vec![existing_widget, tile_cell_placement];
                                Cell::Many(data, widgets)
                            }
                            Cell::Many(data, mut widgets) => {
                                any_cell_on_board = any_cell_on_board || data.is_on_board;
                                widgets.push(tile_cell_placement);
                                Cell::Many(data, widgets)
                            }
                        };
                        cell_ref.set_data(new);
                    }
                };
            }
        }
        if !any_cell_on_board {
            let unused_tile = UnusedTile {
                id: i,
                base: tile_base.clone(),
                name: tile_name.clone(),
            };
            state.unused_tiles.insert(unused_tile);
        }
        Ok(())
    }
}

impl SimplePlacementModel<Polyform<()>, CoordBox> for PlacementModel {
    fn update_pixel_size(
        &mut self,
        total_view_size_pixel: PixelPosition,
        min_cell_size_pixel: u32,
    ) {
        match self {
            PlacementModel::Polyomino(p) => {
                p.update_pixel_size(total_view_size_pixel, min_cell_size_pixel)
            }
            PlacementModel::Hexomino(h) => {
                h.update_pixel_size(total_view_size_pixel, min_cell_size_pixel)
            }
        }
    }

    fn min_area_size(&self) -> PixelPosition {
        match self {
            PlacementModel::Polyomino(p) => p.min_area_size(),
            PlacementModel::Hexomino(h) => h.min_area_size(),
        }
    }

    fn update_tile_pixel_position(&mut self, idx: usize, position: PixelPosition) {
        match self {
            PlacementModel::Polyomino(p) => p.update_tile_pixel_position(idx, position),
            PlacementModel::Hexomino(h) => h.update_tile_pixel_position(idx, position),
        }
    }

    fn update_tile_dragged(&mut self, idx: usize, dragged: bool) {
        match self {
            PlacementModel::Polyomino(p) => p.update_tile_dragged(idx, dragged),
            PlacementModel::Hexomino(h) => h.update_tile_dragged(idx, dragged),
        }
    }

    fn update_tile_shape(&mut self, idx: usize, shape: Polyform<()>) {
        match (self, shape) {
            (PlacementModel::Polyomino(pm), Polyform::Polyomino(po)) => {
                pm.update_tile_shape(idx, po)
            }
            (PlacementModel::Hexomino(pm), Polyform::Hexomino(po)) => pm.update_tile_shape(idx, po),
            _ => unreachable!(),
        }
    }

    fn prototile_coord(&self, idx: usize, pixel_position: PixelPosition) -> Option<CoordBox> {
        match self {
            PlacementModel::Polyomino(p) => p.prototile_coord(idx, pixel_position),
            PlacementModel::Hexomino(h) => h.prototile_coord(idx, pixel_position),
        }
    }

    fn init_hint_tile(&mut self, position_on_board: CoordBox, shape: Polyform<()>) {
        match (self, position_on_board, shape) {
            (PlacementModel::Polyomino(pm), CoordBox::Regular(r), Polyform::Polyomino(po)) => {
                pm.init_hint_tile(r, po)
            }
            (PlacementModel::Hexomino(pm), CoordBox::Hex(r), Polyform::Hexomino(po)) => {
                pm.init_hint_tile(r, po)
            }
            _ => unreachable!(),
        }
    }

    fn remove_hint_tile(&mut self) {
        match self {
            PlacementModel::Polyomino(p) => p.remove_hint_tile(),
            PlacementModel::Hexomino(h) => h.remove_hint_tile(),
        }
    }
}
