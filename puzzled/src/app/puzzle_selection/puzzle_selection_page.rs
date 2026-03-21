use crate::app::puzzle_selection::puzzle_selection_item::PuzzleSelectionItem;
use crate::model::collection::CollectionModel;
use crate::model::puzzle::PuzzleModel;
use adw::gio;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;
use log::debug;
use puzzle_config::ProgressionConfig;

const PUZZLE_SELECTED_SIGNAL_NAME: &str = "puzzle-selected";

mod imp {
    use super::*;
    use crate::components::info_pill::InfoPill;
    use crate::model::puzzle::PuzzleModel;
    use adw::glib::subclass::Signal;
    use std::cell::RefCell;
    use std::sync::OnceLock;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/de/til7701/Puzzled/ui/page/puzzle-selection-page.ui")]
    pub struct PuzzleSelectionPage {
        #[template_child]
        pub puzzle_name_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub puzzle_description_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub collection_info_box: TemplateChild<adw::WrapBox>,
        #[template_child]
        pub puzzle_count_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub author_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub version_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub puzzle_list: TemplateChild<gtk::ListBox>,

        pub collection: RefCell<Option<CollectionModel>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzleSelectionPage {
        const NAME: &'static str = "PuzzleSelectionPage";
        type Type = super::PuzzleSelectionPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PuzzleSelectionPage {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder(PUZZLE_SELECTED_SIGNAL_NAME)
                        .param_types([PuzzleModel::static_type()])
                        .build(),
                ]
            })
        }

        fn constructed(&self) {
            self.parent_constructed();
            self.obj().setup();
        }
    }
    impl WidgetImpl for PuzzleSelectionPage {}
    impl NavigationPageImpl for PuzzleSelectionPage {}
}

glib::wrapper! {
    pub struct PuzzleSelectionPage(ObjectSubclass<imp::PuzzleSelectionPage>)
        @extends gtk::Widget, adw::NavigationPage,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl PuzzleSelectionPage {
    pub(super) fn setup(&self) {
        self.imp().puzzle_list.connect_row_selected({
            let self_clone = self.clone();
            move |_, row| {
                if let Some(row) = row {
                    let row = row.clone().downcast::<PuzzleSelectionItem>().unwrap();
                    let puzzle = row.puzzle();
                    self_clone.emit_puzzle_selected(&puzzle);
                }
            }
        });
    }

    pub fn connect_puzzle_selected<F: Fn(&PuzzleModel) + 'static>(&self, callback: F) {
        self.connect_local(PUZZLE_SELECTED_SIGNAL_NAME, false, move |values| {
            let model = values[1]
                .get::<PuzzleModel>()
                .expect("Failed to get RandomPuzzlePage from signal");
            callback(&model);
            None
        });
    }

    fn emit_puzzle_selected(&self, puzzle: &PuzzleModel) {
        debug!(
            "Emitting puzzle-selected signal for puzzle: {}",
            puzzle.config().id()
        );
        self.emit_by_name::<()>(PUZZLE_SELECTED_SIGNAL_NAME, &[puzzle]);
    }

    pub fn show_collection(&self, collection: &CollectionModel) {
        self.imp().puzzle_list.remove_all();

        self.imp()
            .puzzle_name_label
            .set_label(collection.config().name());
        if let Some(description) = collection.config().description() {
            self.imp().puzzle_description_label.set_label(description);
            self.imp().puzzle_description_label.set_visible(true);
        } else {
            self.imp().puzzle_description_label.set_visible(false);
        }

        let puzzle_count = collection.puzzles().len();
        self.imp()
            .puzzle_count_pill
            .set_label(format!("{}", puzzle_count));
        self.imp()
            .author_pill
            .set_label(collection.config().author().to_string());
        if let Some(version) = collection.config().version() {
            self.imp().version_pill.set_label(version.to_string());
            if self.imp().version_pill.parent().is_none() {
                self.imp()
                    .collection_info_box
                    .append(&self.imp().version_pill.get());
            }
        } else if self.imp().version_pill.parent().is_some() {
            self.imp()
                .collection_info_box
                .remove(&self.imp().version_pill.get());
        }

        for puzzle in collection.puzzles().iter() {
            let row = PuzzleSelectionItem::new(collection, puzzle);
            self.imp().puzzle_list.append(&row);
        }

        match collection.config().progression() {
            ProgressionConfig::Any => {
                self.imp().puzzle_list.add_css_class("boxed-list-separate");
                self.imp().puzzle_list.remove_css_class("boxed-list");
            }
            ProgressionConfig::Sequential => {
                self.imp().puzzle_list.add_css_class("boxed-list");
                self.imp()
                    .puzzle_list
                    .remove_css_class("boxed-list-separate");
            }
        }
    }
}
