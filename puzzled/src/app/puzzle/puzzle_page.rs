use crate::model::extension::PuzzleTypeExtension;
use crate::model::puzzle::PuzzleModel;
use crate::solver::Solver;
use crate::window::PuzzledWindow;
use adw::gio;
use adw::prelude::NavigationPageExt;
use adw::subclass::prelude::*;
use gtk::glib;
use log::debug;

mod imp {
    use super::*;
    use crate::app::puzzle::puzzle_area::PuzzleArea;
    use crate::model::extension::PuzzleTypeExtension;
    use crate::solver::combination_solutions::CombinationsSolver;
    use crate::window::PuzzledWindow;
    use std::cell::{Cell, OnceCell, RefCell};

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

        pub window: OnceCell<PuzzledWindow>,

        pub puzzle: RefCell<Option<PuzzleModel>>,
        pub extension: RefCell<Option<PuzzleTypeExtension>>,
        pub hint_count: Cell<u32>,
        pub combinations_solver: RefCell<CombinationsSolver>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledPuzzlePage {
        const NAME: &'static str = "PuzzledPuzzlePage";
        type Type = PuzzlePage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.install_action("app.puzzle_info", None, |page, _, _| {
                page.show_puzzle_info()
            });
            klass.install_action("app.select_target", None, |page, _, _| {
                page.show_target_selection_dialog()
            });
            klass.install_action("app.hint", None, |page, _, _| page.on_hint_requested());
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PuzzledPuzzlePage {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().post_construct_setup();
        }
    }
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
    pub fn set_window(&self, window: &PuzzledWindow) {
        self.imp()
            .window
            .set(window.clone())
            .expect("Failed to set window for PuzzlePage");
        self.imp().grid.set_window(window.clone());
    }

    pub fn post_construct_setup(&self) {
        let solver = Solver::default();
        self.imp().grid.connect_tile_moved({
            let self_clone = self.clone();
            move || {
                solver.interrupt_solver_call();
                let puzzle_state = self_clone.imp().grid.extract_puzzle_state();
                if let Ok(puzzle_state) = puzzle_state
                    && solver.is_solved(&puzzle_state)
                {
                    self_clone.on_solved();
                }
            }
        });
        self.connect_hiding({
            move |_| {
                Solver::default().interrupt_solver_call();
            }
        });
    }

    pub fn show_puzzle(&self, puzzle: &PuzzleModel) {
        self.imp().puzzle.replace(Some(puzzle.clone()));
        self.update_extension(&Some(PuzzleTypeExtension::default_for_puzzle(
            puzzle.config(),
        )));
        self.imp().hint_count.replace(0);
        self.imp().grid.show_puzzle(puzzle);
        self.show_puzzle_extension();

        let title = format!(
            "{} - {}",
            puzzle.collection().config().name(),
            puzzle.config().name()
        );
        self.set_title(&title);
    }

    pub fn update_extension(&self, extension: &Option<PuzzleTypeExtension>) {
        debug!("Updating puzzle type extension to: {:?}", extension);
        self.imp().extension.replace(extension.clone());
        self.imp().grid.set_puzzle_type_extension(extension.clone());
        self.update_target_selection_button();
    }

    pub fn header_bar(&self) -> adw::HeaderBar {
        self.imp().header_bar.clone()
    }
}
