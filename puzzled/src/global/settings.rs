use adw::gio::Settings;
use adw::prelude::SettingsExt;

pub fn get_settings() -> Settings {
    Settings::new("de.til7701.Puzzled")
}

pub fn get_setting_bool(key: SettingsKey) -> bool {
    let settings = get_settings();
    match key {
        SettingsKey::SolverEnabled => settings.boolean("solver-enabled"),
    }
}

pub enum SettingsKey {
    SolverEnabled,
}
