use crate::adw_ext;
use crate::app::components::polyform_drawing;
use crate::app::components::polyform_drawing::{
    DrawingSettings, PolyformDrawingData, PrototileDrawingMode,
};
use crate::global::settings::{Preferences, ShowBoardGridLines};
use adw::gdk::RGBA;
use adw::gdk::cairo::Context;
use adw::glib;
use adw::subclass::prelude::*;
use adw::{StyleManager, gio};
use gtk::Widget;
use gtk::prelude::*;
use puzzle_config::BoardConfig;
use puzzled_polyform::{CoordBox, MutPrototile, Poly, Polyform};
use std::ops::Deref;

const MAIN_COLORS_COUNT: usize = 6;
const LIGHT_MAIN_COLORS: [RGBA; MAIN_COLORS_COUNT] = [
    adw_ext::LIGHT_2,
    adw_ext::LIGHT_3,
    adw_ext::LIGHT_4,
    adw_ext::LIGHT_5,
    adw_ext::DARK_1,
    adw_ext::DARK_2,
];

const DARK_MAIN_COLORS: [RGBA; MAIN_COLORS_COUNT] = [
    adw_ext::from_hex(0x333333),
    adw_ext::from_hex(0x444444),
    adw_ext::from_hex(0x555555),
    adw_ext::from_hex(0x666666),
    adw_ext::from_hex(0x777777),
    adw_ext::from_hex(0x888888),
];

mod imp {
    use super::*;
    use adw::glib::Properties;
    use std::cell::RefCell;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::BoardView)]
    pub struct PuzzledBoardView {
        #[property(name = "show-grid-lines", get, set)]
        pub show_grid_lines: RefCell<bool>,
        pub layout: RefCell<Option<Polyform<PolyformDrawingData>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledBoardView {
        const NAME: &'static str = "PuzzledBoardView";
        type Type = BoardView;
        type ParentType = gtk::DrawingArea;

        fn class_init(_: &mut Self::Class) {}

        fn instance_init(_: &glib::subclass::InitializingObject<Self>) {}
    }

    #[glib::derived_properties]
    impl ObjectImpl for PuzzledBoardView {}
    impl WidgetImpl for PuzzledBoardView {}
    impl DrawingAreaImpl for PuzzledBoardView {}
}

glib::wrapper! {
    pub struct BoardView(ObjectSubclass<imp::PuzzledBoardView>)
        @extends Widget, gtk::DrawingArea,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl BoardView {
    pub fn new(board_config: &BoardConfig) -> Result<BoardView, String> {
        let obj: BoardView = glib::Object::builder().build();

        let layout = match board_config {
            BoardConfig::Simple { layout } => layout.mapped(&|_| PolyformDrawingData::default()),
            BoardConfig::Area { layout, .. } => layout.mapped(&|value| PolyformDrawingData {
                text: Some(value.display_value.clone()),
                main_color_index: value.area_index as usize,
                ..Default::default()
            }),
        };
        obj.imp().layout.replace(Some(layout));

        obj.set_draw_func({
            let self_clone = obj.clone();
            move |_, cr, width, height| self_clone.draw(cr, width, height)
        });

        let preferences = Preferences::default();
        preferences.bind(ShowBoardGridLines, &obj, "show-grid-lines");
        obj.connect_show_grid_lines_notify({
            move |obj| {
                obj.queue_draw();
            }
        });

        Ok(obj)
    }

    fn draw(&self, cr: &Context, width: i32, height: i32) {
        let layout = self.imp().layout.borrow();
        if let Some(l) = layout.deref() {
            let (main_colors, border_color, text_color) = if StyleManager::default().is_dark() {
                (DARK_MAIN_COLORS, self.dark_border_color(), adw_ext::LIGHT_1)
            } else {
                (
                    LIGHT_MAIN_COLORS,
                    self.light_border_color(),
                    adw_ext::DARK_5,
                )
            };
            let settings = DrawingSettings {
                main_colors,
                highlight_colors: Self::highlight_color,
                border_color,
                text_color,
            };
            let pan = self.create_pango_layout(None);
            polyform_drawing::draw(cr, &pan, width, height, l, &settings);
        }
    }

    fn light_border_color(&self) -> Option<RGBA> {
        if *self.imp().show_grid_lines.borrow() {
            return Some(adw_ext::from_hex(0xfafafb));
        }
        None
    }
    fn dark_border_color(&self) -> Option<RGBA> {
        if *self.imp().show_grid_lines.borrow() {
            return Some(adw_ext::from_hex(0x222226));
        }
        None
    }

    fn highlight_color(drawing_mode: &PrototileDrawingMode) -> Option<RGBA> {
        match drawing_mode {
            PrototileDrawingMode::Normal => None,
            PrototileDrawingMode::Overlapping => None,
            PrototileDrawingMode::OutOfBounds => None,
            PrototileDrawingMode::Highlighted => Some(StyleManager::default().accent_color_rgba()),
        }
    }

    pub fn get_min_element_size(&self) -> u32 {
        10u32
    }

    pub fn highlight(&self, coord: CoordBox) {
        let mut layout = self.imp().layout.borrow_mut();
        if let Some(l) = layout.as_mut() {
            let prototile = l.get_mut(coord);
            if let Some(mut p) = prototile {
                let data = p.data_mut();
                data.drawing_mode = PrototileDrawingMode::Highlighted;
                self.queue_draw();
            }
        }
    }

    pub fn remove_highlights(&self) {
        let mut layout = self.imp().layout.borrow_mut();
        if let Some(l) = layout.as_mut() {
            for mut prototile in l.iter_mut() {
                prototile.data_mut().drawing_mode = PrototileDrawingMode::Normal;
            }
            self.queue_draw();
        }
    }
}
