use crate::model::collection::CollectionModel;
use adw::gio;
use adw::glib;
use adw::subclass::prelude::*;
use gtk::prelude::{ActionableExt, BoxExt, WidgetExt};
use gtk::Widget;
use puzzle_config::PuzzleDifficultyConfig;

mod imp {
    use super::*;
    use crate::components::info_pill::InfoPill;
    use crate::model::collection::CollectionModel;
    use log::debug;
    use std::cell::OnceCell;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/de/til7701/Puzzled/ui/widget/puzzle-collection-item.ui")]
    pub struct PuzzledCollectionSelectionItem {
        #[template_child]
        pub main_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub outer_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub name: TemplateChild<gtk::Label>,
        #[template_child]
        pub info_box: TemplateChild<adw::WrapBox>,
        #[template_child]
        pub puzzle_stars_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub difficulty_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub author_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub version_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub delete_button: TemplateChild<gtk::Button>,

        pub(super) collection: OnceCell<CollectionModel>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledCollectionSelectionItem {
        const NAME: &'static str = "PuzzledCollectionSelectionItem";
        type Type = CollectionSelectionItem;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.install_action("app.delete_community_collection", None, |page, _, _| {
                debug!(
                    "Delete collection action activated for collection '{}'",
                    page.collection().config().name()
                );
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PuzzledCollectionSelectionItem {}
    impl WidgetImpl for PuzzledCollectionSelectionItem {}
    impl ListBoxRowImpl for PuzzledCollectionSelectionItem {}
}

glib::wrapper! {
    pub struct CollectionSelectionItem(ObjectSubclass<imp::PuzzledCollectionSelectionItem>)
        @extends Widget, gtk::ListBoxRow,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap, gtk::Actionable;
}

impl CollectionSelectionItem {
    pub fn new(model: &CollectionModel, core: bool) -> Self {
        let obj: CollectionSelectionItem = glib::Object::builder().build();
        let imp = obj.imp();

        imp.collection
            .set(model.clone())
            .expect("Failed to set collection");

        obj.set_name(model.config().name());

        obj.update_data();

        obj.set_difficulty(model.config().average_difficulty());

        if core {
            obj.set_author(None);
        } else {
            obj.set_author(Some(model.config().author()));
        }

        obj.set_version(model.config().version());

        obj.show_delete_button(!core);
        if !core {
            obj.set_delete_action_target(Some(&model.config().id().to_string().into()));
        }

        model.connect_progress_changed({
            let obj = obj.clone();
            move || {
                obj.update_data();
            }
        });

        obj
    }

    fn update_data(&self) {
        let (stars_reached, stars_total) = self.imp().collection.get().unwrap().stars();
        self.set_star_counts(stars_reached as usize, stars_total as usize);
    }

    fn set_name(&self, name: &str) {
        self.imp().name.set_text(name);
    }

    fn set_star_counts(&self, solved: usize, total: usize) {
        self.imp()
            .puzzle_stars_pill
            .set_label(format!("{} / {}", solved, total));
        self.imp().puzzle_stars_pill.highlight(solved == total);
    }

    fn set_difficulty(&self, difficulty: Option<PuzzleDifficultyConfig>) {
        if let Some(difficulty) = difficulty {
            let text: String = difficulty.into();
            self.imp().difficulty_pill.set_label(text);
            if self.imp().difficulty_pill.parent().is_none() {
                self.imp()
                    .info_box
                    .insert_before(&self.imp().difficulty_pill.get(), None::<&Widget>);
            }
        } else {
            self.imp()
                .info_box
                .remove(&self.imp().difficulty_pill.get());
        }
    }

    fn set_author(&self, author: Option<&str>) {
        if let Some(author) = author {
            self.imp().author_pill.set_label(author);
            if self.imp().author_pill.parent().is_none() {
                self.imp()
                    .info_box
                    .insert_before(&self.imp().author_pill.get(), None::<&Widget>);
            }
        } else {
            self.imp().info_box.remove(&self.imp().author_pill.get());
        }
    }

    fn set_version(&self, version: &Option<String>) {
        if let Some(version) = version {
            self.imp().version_pill.set_label(version.as_str());
            if self.imp().version_pill.parent().is_none() {
                self.imp()
                    .info_box
                    .insert_before(&self.imp().version_pill.get(), None::<&Widget>);
            }
        } else {
            self.imp().info_box.remove(&self.imp().version_pill.get());
        }
    }

    fn show_delete_button(&self, show: bool) {
        if show {
            if self.imp().delete_button.get().parent().is_none() {
                self.imp()
                    .main_box
                    .insert_after(&self.imp().delete_button.get(), None::<&Widget>);
            }
        } else {
            self.imp().main_box.remove(&self.imp().delete_button.get());
        }
    }

    fn set_delete_action_target(&self, target_value: Option<&glib::Variant>) {
        self.imp()
            .delete_button
            .set_action_target_value(target_value);
    }

    pub fn collection(&self) -> &CollectionModel {
        self.imp()
            .collection
            .get()
            .expect("Collection should be set for CollectionSelectionItem")
    }
}
