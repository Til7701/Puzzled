use adw::gio;
use adw::glib;
use adw::subclass::prelude::*;
use gtk::prelude::{BoxExt, WidgetExt};
use gtk::Widget;
use puzzle_config::PuzzleDifficultyConfig;

mod imp {
    use super::*;
    use crate::view::info_pill::InfoPill;
    use std::cell::Cell;

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
        pub puzzle_solved_pill: TemplateChild<InfoPill>,
        pub solved_count: Cell<(usize, usize)>,
        #[template_child]
        pub difficulty_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub author_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub version_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub delete_button: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledCollectionSelectionItem {
        const NAME: &'static str = "PuzzledCollectionSelectionItem";
        type Type = CollectionSelectionItem;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
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
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_name(&self, name: &str) {
        self.imp().name.set_text(name);
    }

    pub fn set_solved_counts(&self, solved: usize, total: usize) {
        self.imp()
            .puzzle_solved_pill
            .set_label(format!("{} / {}", solved, total));
        self.imp().solved_count.set((solved, total));
    }

    pub fn increment_solved_count(&self) {
        let mut counts = self.imp().solved_count.get();
        counts.0 += 1;
        self.set_solved_counts(counts.0, counts.1);
    }

    pub fn set_difficulty(&self, difficulty: Option<PuzzleDifficultyConfig>) {
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

    pub fn set_author(&self, author: Option<&str>) {
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

    pub fn set_version(&self, version: &Option<String>) {
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

    pub fn show_delete_button(&self, show: bool) {
        if show && self.imp().delete_button.parent().is_none() {
            self.imp()
                .main_box
                .insert_after(&self.imp().delete_button.get(), None::<&Widget>);
        } else {
            self.imp().main_box.remove(&self.imp().delete_button.get());
        }
    }
}
