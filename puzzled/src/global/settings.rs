use adw::gio::Settings;
use adw::glib;
use adw::prelude::{IsA, SettingsExt, SettingsExtManual};

/// A reusable container for preferences/settings access.
#[derive(Debug, Clone)]
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
    pub fn get<S: SettingKey>(&self, setting: S) -> S::Value {
        setting.get(&self.settings)
    }

    /// Binds the given setting to the specified property of the given object.
    pub fn bind<S: SettingKey>(&self, setting: S, obj: &impl IsA<glib::Object>, property: &str) {
        setting.bind(&self.settings, obj, property);
    }
}

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

pub struct ShowBoardGridLines;

impl SettingKey for ShowBoardGridLines {
    type Value = bool;

    fn key(&self) -> &'static str {
        "show-board-grid-lines"
    }

    fn get(&self, settings: &Settings) -> Self::Value {
        settings.boolean(self.key())
    }
}
