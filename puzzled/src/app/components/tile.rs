use crate::adw_ext;
use crate::app::components::polyform_drawing;
use crate::app::components::polyform_drawing::{
    DrawingSettings, PolyformDrawingData, PolyformDrawingDataSpecial, PrototileDrawingMode,
};
use adw::gdk::RGBA;
use adw::gio;
use adw::glib;
use adw::subclass::prelude::*;
use gtk::cairo::Context;
use gtk::prelude::{DrawingAreaExtManual, WidgetExt};
use puzzle_config::ColorConfig;
use puzzled_common::polyform::{CoordBox, MutPrototile, Poly, Polyform, Prototile, RegularCoord};
use std::ops::DerefMut;

const HIGHLIGHT_OVERLAPPING_COLOR: RGBA = adw_ext::ERROR_BG_LIGHT;
const HIGHLIGHT_OUT_OF_BOUNDS_COLOR: RGBA = adw_ext::WARNING_BG_LIGHT;

mod imp {
    use super::*;
    use crate::app::components::polyform_drawing::PolyformDrawingData;
    use puzzled_common::polyform::{CoordBox, Poly, RegularCoord};
    use std::cell::{Cell, RefCell};
    use std::ops::Deref;

    #[derive(Debug, Default)]
    pub struct PuzzledTileView {
        pub id: Cell<usize>,
        pub current_rotation: RefCell<Polyform<PolyformDrawingData>>,
        pub color: Cell<Option<RGBA>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledTileView {
        const NAME: &'static str = "PuzzledTileView";
        type Type = TileView;
        type ParentType = gtk::DrawingArea;

        fn class_init(_: &mut Self::Class) {}

        fn instance_init(_: &glib::subclass::InitializingObject<Self>) {}
    }

    impl ObjectImpl for PuzzledTileView {}
    impl WidgetImpl for PuzzledTileView {
        fn contains(&self, x: f64, y: f64) -> bool {
            if x < 0.0 || y < 0.0 {
                return false;
            }

            let obj = self.obj();
            let width = obj.width() as f64;
            if x > width {
                return false;
            }
            let height = obj.height() as f64;
            if y > height {
                return false;
            }

            let current_rotation = self.current_rotation.borrow();
            match current_rotation.deref() {
                Polyform::Polyomino(polyomino) => {
                    let tile_dims = polyomino.dim();
                    let cell_width = width / tile_dims.x() as f64;
                    let cell_height = height / tile_dims.y() as f64;

                    let cell_x = (x / cell_width) as i32;
                    let cell_y = (y / cell_height) as i32;

                    current_rotation
                        .get(CoordBox::Regular(RegularCoord::new(cell_x, cell_y)))
                        .is_some()
                }
                Polyform::Hexomino { .. } => {
                    todo!()
                }
            }
        }
    }
    impl DrawingAreaImpl for PuzzledTileView {}
}

glib::wrapper! {
    pub struct TileView(ObjectSubclass<imp::PuzzledTileView>)
        @extends gtk::Widget, gtk::DrawingArea,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl TileView {
    /// Creates a new TileView with the given id and base layout.
    /// The name is used to refer to the tile layout when calculating possible solutions for given
    /// tiles.
    pub fn new(id: usize, base: Polyform<()>, color: ColorConfig) -> Self {
        let obj: TileView = glib::Object::builder().build();

        obj.imp().id.replace(id);
        obj.imp()
            .current_rotation
            .replace(base.mapped(&|_| PolyformDrawingData::default()));
        obj.update_current_rotation_specials();
        obj.imp().color.replace(Some(RGBA::new(
            color.red() as f32 / 255.0,
            color.green() as f32 / 255.0,
            color.blue() as f32 / 255.0,
            color.alpha() as f32 / 255.0,
        )));

        obj.set_draw_func({
            let self_clone = obj.clone();
            move |_, cr, width, height| self_clone.draw(cr, width, height)
        });

        obj
    }

    fn draw(&self, cr: &Context, width: i32, height: i32) {
        let current_rotation = self.imp().current_rotation.borrow();
        let settings = DrawingSettings {
            main_colors: [self.color()],
            highlight_colors: Self::highlight_color,
            border_color: None,
            text_color: adw_ext::DARK_5, // Does not matter
        };
        let pan = self.create_pango_layout(None);
        polyform_drawing::draw(cr, &pan, width, height, &current_rotation, &settings);
    }

    fn highlight_color(drawing_mode: &PrototileDrawingMode) -> Option<RGBA> {
        match drawing_mode {
            PrototileDrawingMode::Normal => None,
            PrototileDrawingMode::Overlapping => Some(HIGHLIGHT_OVERLAPPING_COLOR),
            PrototileDrawingMode::OutOfBounds => Some(HIGHLIGHT_OUT_OF_BOUNDS_COLOR),
            PrototileDrawingMode::Highlighted => None,
        }
    }

    /// Returns the id of the tile to identify it.
    pub fn id(&self) -> usize {
        self.imp().id.get()
    }

    pub fn color(&self) -> RGBA {
        self.imp().color.get().unwrap()
    }

    fn update_current_rotation_specials(&self) {
        let current_rotation_lookup = self.imp().current_rotation.borrow().clone(); // TODO avoid
        let mut current_rotation = self.imp().current_rotation.borrow_mut();
        for mut prototile in current_rotation.deref_mut().iter_mut() {
            let coord = prototile.coord();
            let data = prototile.data_mut();
            match (coord, &mut data.special) {
                (CoordBox::Regular(c), PolyformDrawingDataSpecial::Polyomino(s)) => {
                    if c.x() > 0
                        && current_rotation_lookup
                            .get(CoordBox::Regular(c - RegularCoord::new(1, 0)))
                            .is_some()
                    {
                        s.x_neg_neighbour = true;
                    }
                    if current_rotation_lookup
                        .get(CoordBox::Regular(c + RegularCoord::new(1, 0)))
                        .is_some()
                    {
                        s.x_pos_neighbour = true;
                    }
                    if c.y() > 0
                        && current_rotation_lookup
                            .get(CoordBox::Regular(c - RegularCoord::new(0, 1)))
                            .is_some()
                    {
                        s.y_neg_neighbour = true;
                    }
                    if current_rotation_lookup
                        .get(CoordBox::Regular(c + RegularCoord::new(0, 1)))
                        .is_some()
                    {
                        s.y_pos_neighbour = true;
                    }
                }
                (CoordBox::Hex(_), PolyformDrawingDataSpecial::Hexomino()) => {
                    todo!()
                }
                _ => unreachable!(),
            }
        }
    }

    /// Rotates the tile one step clockwise.
    pub fn rotate_clockwise(&self, prime: CoordBox) {
        // We are calling counterclockwise here, since the tile is drawn transposed.
        self.imp()
            .current_rotation
            .borrow_mut()
            .rotate_around(prime);
        self.update_current_rotation_specials();
    }

    /// Flips the tile horizontally.
    pub fn flip(&self, prime: CoordBox) {
        self.imp().current_rotation.borrow_mut().flip(prime);
        self.update_current_rotation_specials();
    }

    /// Returns the current layout of the tile, which changes when the tile is rotated or flipped.
    pub fn current_rotation(&self) -> Polyform<()> {
        self.imp().current_rotation.borrow().mapped(&|_| ())
    }

    /// Sets the drawing mode for the cell at the given coordinates.
    pub fn set_drawing_mode_at(&self, coord: CoordBox, drawing_mode: PrototileDrawingMode) {
        if let Some(mut prototile) = self.imp().current_rotation.borrow_mut().get_mut(coord) {
            prototile.data_mut().drawing_mode = drawing_mode;
        }
        self.queue_draw();
    }

    /// Resets the drawing mode for all cells to [PrototileDrawingMode::Normal].
    pub fn reset_drawing_modes(&self) {
        for mut prototile in self.imp().current_rotation.borrow_mut().iter_mut() {
            prototile.data_mut().drawing_mode = PrototileDrawingMode::Normal;
        }
        self.queue_draw();
    }
}
