use crate::model::puzzle::PuzzleModel;
use adw::gio;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

mod imp {
    use super::*;
    use crate::app::puzzle_area::puzzle_area::puzzle_area::PuzzleArea;
    use crate::model::extension::PuzzleTypeExtension;
    use std::cell::RefCell;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/de/til7701/Puzzled/ui/page/puzzle-page.ui")]
    pub struct PuzzledPuzzlePage {
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        #[template_child]
        pub grid: TemplateChild<PuzzleArea>,
        #[template_child]
        pub puzzle_info_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub extension_separator: TemplateChild<gtk::Separator>,
        #[template_child]
        pub target_selection_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub hint_button: TemplateChild<gtk::Button>,

        pub puzzle: RefCell<Option<PuzzleModel>>,
        pub extension: RefCell<Option<PuzzleTypeExtension>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledPuzzlePage {
        const NAME: &'static str = "PuzzledPuzzleAreaPage";
        type Type = PuzzlePage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.install_action("app.puzzle_info", None, |page, _, _| {
                page.show_puzzle_info()
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PuzzledPuzzlePage {}
    impl WidgetImpl for PuzzledPuzzlePage {}
    impl NavigationPageImpl for PuzzledPuzzlePage {}
}

glib::wrapper! {
    pub struct PuzzlePage(ObjectSubclass<imp::PuzzledPuzzlePage>)
        @extends gtk::Widget, adw::NavigationPage,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl PuzzlePage {
    pub fn show_puzzle(&self, puzzle: &PuzzleModel) {
        self.imp().puzzle.replace(Some(puzzle.clone()));
        self.imp().grid.show_puzzle(puzzle);
    }
}
