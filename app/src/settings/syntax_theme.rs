use serde::{Deserialize, Serialize};
use settings::macros::define_settings_group;
use settings::{RespectUserSyncSetting, SupportedPlatforms, SyncToCloud};
use syntax_tree::{ColorMap, DARK_MODERN, LIGHT_MODERN};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(description = "The syntax highlighting color theme.", rename_all = "snake_case")]
pub enum SyntaxThemeKind {
    /// Derives syntax colors from the active app color theme's terminal
    /// palette, matching Warp's original (pre-syntax-theme) behavior.
    #[default]
    #[schemars(description = "Match App Theme")]
    MatchAppTheme,
    #[schemars(description = "Dark Modern")]
    DarkModern,
    #[schemars(description = "Light Modern")]
    LightModern,
}

impl SyntaxThemeKind {
    pub fn fixed_color_map(self) -> Option<ColorMap> {
        match self {
            SyntaxThemeKind::MatchAppTheme => None,
            SyntaxThemeKind::DarkModern => Some(DARK_MODERN),
            SyntaxThemeKind::LightModern => Some(LIGHT_MODERN),
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            SyntaxThemeKind::MatchAppTheme => "Match App Theme",
            SyntaxThemeKind::DarkModern => "Dark Modern",
            SyntaxThemeKind::LightModern => "Light Modern",
        }
    }
}

define_settings_group!(SyntaxThemeSettings, settings: [
    syntax_theme_kind: SyntaxTheme {
        type: SyntaxThemeKind,
        default: SyntaxThemeKind::default(),
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "appearance.themes.syntax_theme",
        max_table_depth: 0,
        description: "The syntax highlighting color theme, independent of the app color theme.",
    },
]);
