use crate::model::collection::CollectionModel;
use crate::model::puzzle_meta::PuzzleMeta;
use adw::gio;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;
use log::debug;
use puzzle_config::random::{random_puzzle, Algorithm, RandomPuzzleSettings};
use std::hash::{DefaultHasher, Hash, Hasher};

const CREATE_RANDOM_PUZZLE_SIGNAL_NAME: &str = "random-puzzle-created";

mod imp {
    use super::*;
    use crate::model::collection::CollectionModel;
    use adw::glib::subclass::Signal;
    use std::sync::OnceLock;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/de/til7701/Puzzled/ui/page/random-puzzle-page.ui")]
    pub struct PuzzledRandomPuzzlePage {
        #[template_child]
        pub seed_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub tile_count_row: TemplateChild<adw::SpinRow>,
        #[template_child]
        pub board_width_row: TemplateChild<adw::SpinRow>,
        #[template_child]
        pub board_height_row: TemplateChild<adw::SpinRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledRandomPuzzlePage {
        const NAME: &'static str = "PuzzledRandomPuzzlePage";
        type Type = RandomPuzzlePage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.install_action("app.create_random_puzzle", None, move |page, _, _| {
                page.show_random_puzzle();
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PuzzledRandomPuzzlePage {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder(CREATE_RANDOM_PUZZLE_SIGNAL_NAME)
                        .param_types([CollectionModel::static_type()])
                        .build(),
                ]
            })
        }
    }
    impl WidgetImpl for PuzzledRandomPuzzlePage {}
    impl NavigationPageImpl for PuzzledRandomPuzzlePage {}
}

glib::wrapper! {
    pub struct RandomPuzzlePage(ObjectSubclass<imp::PuzzledRandomPuzzlePage>)
        @extends gtk::Widget, adw::NavigationPage,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl RandomPuzzlePage {
    pub fn connect_create_random_puzzle<F: Fn(&CollectionModel) + 'static>(&self, callback: F) {
        self.connect_local(CREATE_RANDOM_PUZZLE_SIGNAL_NAME, false, move |values| {
            let page = values[1]
                .get::<CollectionModel>()
                .expect("Failed to get CollectionModel from signal");
            callback(&page);
            None
        });
    }

    fn show_random_puzzle(&self) {
        debug!("Setting random puzzle");
        let settings = RandomPuzzleSettings {
            seed: self.get_seed(),
            algorithm: Algorithm::Growing {
                tile_count: self.imp().tile_count_row.value() as usize,
                board_width: self.imp().board_width_row.value() as usize,
                board_height: self.imp().board_height_row.value() as usize,
            },
        };
        let collection = random_puzzle(&settings);
        let collection = CollectionModel::new(collection, &PuzzleMeta::new());
        collection.mark_all_as_unsolved();
        debug!("Generated random puzzle collection");
        self.emit_by_name::<()>(CREATE_RANDOM_PUZZLE_SIGNAL_NAME, &[&collection]);
    }

    fn get_seed(&self) -> u64 {
        let text = self.imp().seed_entry.text();
        if text.is_empty() {
            return rand::random();
        }
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        hasher.finish()
    }
}
