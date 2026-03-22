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
    use crate::app::components::info_pill::InfoPill;
    use crate::model::puzzle::PuzzleModel;
    use adw::glib::subclass::Signal;
    use adw::glib::VariantTy;
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
            klass.install_action(
                "app.puzzle_activated",
                Some(VariantTy::INT32),
                |page, _, v| {
                    if let Some(variant) = v {
                        let index = variant
                            .get::<i32>()
                            .expect("Failed to get index from variant")
                            as usize;
                        let collection = page.imp().collection.borrow();
                        let puzzle = collection
                            .as_ref()
                            .expect("No collection set in PuzzleSelectionPage")
                            .puzzles()
                            .get(index)
                            .expect("Index out of bounds in puzzle list");
                        page.emit_puzzle_selected(puzzle);
                    }
                },
            );
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
    /// Connects to the `puzzle_selected` signal.
    /// This signal is emitted, when the user selects a puzzle.
    /// The puzzle area should be shown when this happens.
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

    /// Shows the given collection in the view. This means, it shows all puzzles of the collection
    /// and displays the collection information and the puzzles which the user can select.
    /// If that is the case, the `puzzle_selected` signal is emitted.
    ///
    /// # Arguments
    ///
    /// * `collection`: the collection to show
    ///
    /// returns: ()
    pub fn show_collection(&self, collection: &CollectionModel) {
        self.imp().collection.replace(Some(collection.clone()));
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
            let row = PuzzleSelectionItem::new(puzzle);
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
