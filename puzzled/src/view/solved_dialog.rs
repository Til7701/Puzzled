use adw::gio;
use adw::glib;
use adw::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/de/til7701/Puzzled/ui/dialog/solved-dialog.ui")]
    pub struct PuzzledSolvedDialog {}

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledSolvedDialog {
        const NAME: &'static str = "PuzzledSolvedDialog";
        type Type = SolvedDialog;
        type ParentType = adw::AlertDialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PuzzledSolvedDialog {}
    impl WidgetImpl for PuzzledSolvedDialog {}
    impl AdwDialogImpl for PuzzledSolvedDialog {}
    impl AdwAlertDialogImpl for PuzzledSolvedDialog {}
}

glib::wrapper! {
    pub struct SolvedDialog(ObjectSubclass<imp::PuzzledSolvedDialog>)
        @extends gtk::Widget, adw::Dialog, adw::AlertDialog,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl SolvedDialog {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
