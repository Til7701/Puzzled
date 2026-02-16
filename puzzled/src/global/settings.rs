use adw::gio::Settings;
use adw::glib;
use adw::prelude::{IsA, SettingsExt, SettingsExtManual};

/// A reusable container for preferences/settings access.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Preferences {
    settings: Settings,
}

impl Default for Preferences {
    fn default() -> Self {
        Preferences {
            settings: Settings::new("de.til7701.Puzzled"),
        }
    }
}

impl Preferences {
    /// Returns the value of the given setting.
    #[allow(dead_code)]
    pub fn get<S: SettingKey>(&self, setting: S) -> S::Value {
        setting.get(&self.settings)
    }

    /// Binds the given setting to the specified property of the given object.
    #[allow(dead_code)]
    pub fn bind<S: SettingKey>(&self, setting: S, obj: &impl IsA<glib::Object>, property: &str) {
        setting.bind(&self.settings, obj, property);
    }
}

#[allow(dead_code)]
pub trait SettingKey {
    type Value;

    /// Returns the key of the setting.
    /// This must be the same as defined in the GSettings schema.
    fn key(&self) -> &'static str;

    /// Returns the value of the setting from the given `Settings` object.
    fn get(&self, settings: &Settings) -> Self::Value;

    /// Binds the setting to the specified property of the given object.
    fn bind(&self, settings: &Settings, obj: &impl IsA<glib::Object>, property: &str) {
        settings.bind(self.key(), obj, property).build();
    }
}

#[allow(dead_code)]
pub struct SolverEnabled;

impl SettingKey for SolverEnabled {
    type Value = bool;

    fn key(&self) -> &'static str {
        "solver-enabled"
    }

    fn get(&self, settings: &Settings) -> Self::Value {
        settings.boolean(self.key())
    }
}
