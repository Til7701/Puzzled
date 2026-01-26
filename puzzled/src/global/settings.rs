use adw::gio::Settings;
use adw::glib;
use adw::prelude::{IsA, SettingsExt, SettingsExtManual};

fn get_settings() -> Settings {
    Settings::new("de.til7701.Puzzled")
}

pub trait SettingKey {
    type Value;

    fn key(&self) -> &'static str;
    fn get(&self) -> Self::Value;
    fn bind(&self, obj: &impl IsA<glib::Object>, property: &str) {
        get_settings().bind(self.key(), obj, property).build();
    }
}

pub struct SolverEnabled;

impl SettingKey for SolverEnabled {
    type Value = bool;
    fn key(&self) -> &'static str {
        "solver-enabled"
    }
    fn get(&self) -> Self::Value {
        get_settings().boolean(self.key())
    }
}
