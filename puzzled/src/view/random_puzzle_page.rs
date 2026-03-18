use crate::global::state::get_state_mut;
use crate::puzzles;
use adw::gio;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;
use log::debug;
use puzzle_config::random;
use puzzle_config::random::RandomPuzzleSettings;
use std::hash::{DefaultHasher, Hash, Hasher};

const CREATE_RANDOM_PUZZLE_SIGNAL_NAME: &str = "random-puzzle-created";

mod imp {
    use super::*;
    use adw::glib::subclass::Signal;
    use std::sync::OnceLock;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/de/til7701/Puzzled/ui/page/random-puzzle-page.ui")]
    pub struct PuzzledRandomPuzzlePage {
        #[template_child]
        pub seed_entry: TemplateChild<adw::EntryRow>,
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
                        .param_types([RandomPuzzlePage::static_type()])
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
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn connect_create_random_puzzle<F: Fn(&RandomPuzzlePage) + 'static>(&self, callback: F) {
        self.connect_local(CREATE_RANDOM_PUZZLE_SIGNAL_NAME, false, move |values| {
            let page = values[0]
                .get::<RandomPuzzlePage>()
                .expect("Failed to get RandomPuzzlePage from signal");
            callback(&page);
            None
        });
    }

    fn show_random_puzzle(&self) {
        debug!("Setting random puzzle");
        let predefined = puzzles::get_predefined();
        let settings = RandomPuzzleSettings {
            seed: self.get_seed(),
            tile_count: 10,
            tiles: predefined.tiles(),
        };
        let collection = random::random_puzzle(&settings);
        debug!("Generated random puzzle collection");
        let mut state = get_state_mut();
        state.setup_for_puzzle(collection.puzzles().first().unwrap().clone());
        state.puzzle_collection = Some(collection);
        drop(state);
        self.emit_by_name::<()>(CREATE_RANDOM_PUZZLE_SIGNAL_NAME, &[self]);
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
