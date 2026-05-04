use crate::model::collection::CollectionModel;
use crate::model::puzzle::PuzzleModel;
use crate::offset::PixelOffset;
use adw::glib;
use adw::prelude::ObjectExt;
use adw::subclass::prelude::*;
use log::debug;

mod grid;
mod initial;

const TILE_MOVED_SIGNAL_NAME: &str = "tile-moved";

mod imp {
    use super::*;
    use crate::model::extension::PuzzleTypeExtension;
    use crate::model::placement::grid::GridConfig;
    use crate::offset::PixelOffset;
    use adw::glib::subclass::Signal;
    use adw::glib::Properties;
    use std::cell::{Cell, RefCell};
    use std::sync::OnceLock;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::PlacementModel)]
    pub struct PuzzledPlacementModel {
        pub(super) pixel_size: Cell<PixelOffset>,
        pub(super) grid_config: RefCell<GridConfig>,
        pub(super) puzzle_type_extension: RefCell<Option<PuzzleTypeExtension>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledPlacementModel {
        const NAME: &'static str = "PuzzledPlacementModel";
        type Type = PlacementModel;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for PuzzledPlacementModel {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder(TILE_MOVED_SIGNAL_NAME).build()])
        }
    }
}

glib::wrapper! {
    pub struct PlacementModel(ObjectSubclass<imp::PuzzledPlacementModel>);
}

impl PlacementModel {
    pub fn new(puzzle_model: PuzzleModel) -> Self {
        let obj: PlacementModel = glib::Object::builder().build();
        obj.update_pixel_size(PixelOffset(100.0, 100.0));
        obj
    }

    pub fn update_pixel_size(&self, size: PixelOffset) {
        self.imp().pixel_size.replace(size);
    }

    pub fn connect_tile_moved<F: Fn() + 'static>(&self, callback: F) {
        self.connect_local(TILE_MOVED_SIGNAL_NAME, false, move |_| {
            callback();
            None
        });
    }

    fn emit_tile_moved(&self) {
        debug!("Emitting tile moved signal",);
        self.emit_by_name::<()>(TILE_MOVED_SIGNAL_NAME, &[]);
    }
}
