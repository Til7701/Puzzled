use crate::app::components::tile::TileView;
use crate::app::puzzle::puzzle_area::PuzzleArea;
use crate::offset::CellOffset;
use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{FixedExt, WidgetExt};
use puzzle_config::ColorConfig;
use puzzle_solver::result::TilePlacement;

impl PuzzleArea {
    /// Show the placement of a tile as a hint.
    pub fn show_hint_tile(&self, placement: &TilePlacement) {
        let tiles = self.imp().tiles.borrow();
        let tile_matching_base = {
            let placement_model = self.imp().placement_model.borrow();
            let index = placement_model
                .as_ref()
                .unwrap()
                .find_tile_matching_base(placement.base());
            index.and_then(|i| tiles.get(i))
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
        let tile_view = TileView::new(usize::MAX, placement.rotation().clone(), color_config);

        self.imp()
            .placement_model
            .borrow()
            .as_ref()
            .unwrap()
            .init_hint_tile(
                CellOffset::from(placement.position()) - CellOffset(1, 1), // Plus 1, 1 because the puzzle state has a border of one cell to provide information for highlighting
                placement.rotation().clone(),
            );

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
            let placement_borrow = self.imp().placement_model.borrow();
            let placement_model = placement_borrow.as_ref().unwrap();
            let pos = placement_model.hint_tile_position();
            let size = placement_model.hint_tile_size();

            tile_view.set_width_request(size.0 as i32);
            tile_view.set_height_request(size.1 as i32);
            self.move_(tile_view, pos.0, pos.1);
        }
    }
}
