use crate::app::components::tile::TileView;
use crate::app::puzzle::puzzle_area::PuzzleArea;
use crate::offset::PixelOffset;
use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{FixedExt, WidgetExt};
use puzzle_config::ColorConfig;
use puzzle_solver::result::TilePlacement;

impl PuzzleArea {
    /// Show the placement of a tile as a hint.
    pub fn show_hint_tile(&self, placement: &TilePlacement) {
        let tile_matching_base = {
            let tiles = self.imp().tiles.borrow();
            tiles
                .iter()
                .find(|t| t.base().eq(placement.base()))
                .cloned()
        };
        if tile_matching_base.is_none() {
            return;
        }
        let tile_matching_base = tile_matching_base.unwrap();
        let color = tile_matching_base.color().with_alpha(0.5);
        let color_config = ColorConfig::new(
            (color.red() * 255.0) as u8,
            (color.green() * 255.0) as u8,
            (color.blue() * 255.0) as u8,
            (color.alpha() * 255.0) as u8,
        );
        let tile_view = self.create_hint_tile(placement, color_config);
        self.remove_hint_tile();
        self.imp().hint_tile.replace(Some(tile_view.clone()));
        self.put(&tile_view, 0.0, 0.0);
        self.update_layout();
    }

    fn create_hint_tile(&self, placement: &TilePlacement, color_config: ColorConfig) -> TileView {
        let tile_view = TileView::new(usize::MAX, placement.rotation().clone(), color_config, None);

        self.imp()
            .placement_model
            .borrow()
            .as_ref()
            .unwrap()
            .init_hint_tile_position(placement.position().into());

        let click_gesture = gtk::GestureClick::new();
        click_gesture.connect_pressed({
            let self_clone = self.clone();
            move |_, _, _, _| {
                self_clone.remove_hint_tile();
            }
        });
        tile_view.add_controller(click_gesture);

        tile_view
    }

    /// Remove the hint tile from the puzzle area, if one is currently shown.
    pub fn remove_hint_tile(&self) {
        if let Some(tile_view) = self.imp().hint_tile.replace(None) {
            self.remove(&tile_view);
            self.imp()
                .placement_model
                .borrow()
                .as_ref()
                .unwrap()
                .remove_hint_tile();
        }
    }

    pub fn update_hint_tile_layout(&self) {
        let hint_tile = self.imp().hint_tile.borrow();
        if let Some(tile_view) = hint_tile.as_ref() {
            let grid_size = grid_config.cell_size_pixel;
            let dims = tile_view.current_rotation().dim();
            tile_view.set_width_request(dims.0 as i32 * grid_size as i32);
            tile_view.set_height_request(dims.1 as i32 * grid_size as i32);

            if let Some(position_cells) = tile_view.position_cells() {
                let pos: PixelOffset = position_cells.mul_scalar(grid_size as f64).into();
                self.move_(tile_view, pos.0, pos.1);
                tile_view.set_position_pixels(pos);
            }
        }
    }
}
