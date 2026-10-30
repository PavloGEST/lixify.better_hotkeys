use rdev::Key;
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub window_features: WindowFeatures,
}

#[derive(Debug, Deserialize)]
pub struct WindowFeatures {
    pub set_window_fullscreen: FeatureConfig,
    pub quick_close_window: FeatureConfig,
    pub slide_to_desktop: FeatureConfig,
    pub move_window_to_desktop: FeatureConfig,
    pub drag_window: FeatureConfig,
}

#[derive(Debug, Deserialize)]
pub struct FeatureConfig {
    pub active: bool,
    #[serde(deserialize_with = "crate::keys::deserialize_key_bindings")]
    pub key_bindings: Vec<Key>,
    pub additional_flags: Option<Vec<String>>, // For example, for move_window_to_desktop automatical sliding to the desktop, e.g. ["auto_slide"] 
}

#[derive(Debug, Clone)]
pub struct ActiveWindowFeature<'a> {
    pub name: &'static str,
    pub config: &'a FeatureConfig,
    pub function: FeatureAction,
}

#[derive(Debug, Clone, Copy)]
pub enum FeatureAction {

    Plain(fn()),

    Numbered(fn(u32)),

    Drag,
}

impl Config {
    pub fn get_active_window_features(&self) -> Vec<ActiveWindowFeature<'_>> {
        [
            ActiveWindowFeature {
                name: "set_window_fullscreen",
                config: &self.window_features.set_window_fullscreen,
                function: FeatureAction::Plain(crate::window_features::set_window_fullscreen),
            },
            ActiveWindowFeature {
                name: "quick_close_window",
                config: &self.window_features.quick_close_window,
                function: FeatureAction::Plain(crate::window_features::quick_close_window),
            },
            ActiveWindowFeature {
                name: "slide_to_desktop",
                config: &self.window_features.slide_to_desktop,
                function: FeatureAction::Numbered(crate::window_features::slide_to_desktop),
            },
            ActiveWindowFeature {
                name: "move_window_to_desktop",
                config: &self.window_features.move_window_to_desktop,
                function: FeatureAction::Numbered(crate::window_features::move_window_to_desktop),
            },
            ActiveWindowFeature {
                name: "drag_window",
                config: &self.window_features.drag_window,
                function: FeatureAction::Drag,
            },

        ]
        .into_iter()
        .filter(|feature| feature.config.active)
        .collect()
    }
}

pub fn load(path: &str) -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    Ok(toml::from_str(&contents)?)
}
