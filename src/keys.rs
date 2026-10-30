
use serde::Deserialize;
use serde::de::Deserializer;
use rdev::Key;
use crate::config::{ActiveWindowFeature, FeatureAction};
use std::collections::HashSet;

struct KeyDefinition {
    name: &'static str,
    code: Key,
}

const KEY_DEFINITIONS: &[KeyDefinition] = &[
    KeyDefinition {
        name: "KeyA",
        code: Key::KeyA,
    },
    KeyDefinition {
        name: "KeyB",
        code: Key::KeyB,
    },
    KeyDefinition {
        name: "KeyC",
        code: Key::KeyC,
    },
    KeyDefinition {
        name: "KeyD",
        code: Key::KeyD,
    },
    KeyDefinition {
        name: "KeyE",
        code: Key::KeyE,
    },
    KeyDefinition {
        name: "KeyF",
        code: Key::KeyF,
    },
    KeyDefinition {
        name: "KeyG",
        code: Key::KeyG,
    },
    KeyDefinition {
        name: "KeyH",
        code: Key::KeyH,
    },
    KeyDefinition {
        name: "KeyI",
        code: Key::KeyI,
    },
    KeyDefinition {
        name: "KeyJ",
        code: Key::KeyJ,
    },
    KeyDefinition {
        name: "KeyK",
        code: Key::KeyK,
    },
    KeyDefinition {
        name: "KeyL",
        code: Key::KeyL,
    },
    KeyDefinition {
        name: "KeyM",
        code: Key::KeyM,
    },
    KeyDefinition {
        name: "KeyN",
        code: Key::KeyN,
    },
    KeyDefinition {
        name: "KeyO",
        code: Key::KeyO,
    },
    KeyDefinition {
        name: "KeyP",
        code: Key::KeyP,
    },
    KeyDefinition {
        name: "KeyQ",
        code: Key::KeyQ,
    },
    KeyDefinition {
        name: "KeyR",
        code: Key::KeyR,
    },
    KeyDefinition {
        name: "KeyS",
        code: Key::KeyS,
    },
    KeyDefinition {
        name: "KeyT",
        code: Key::KeyT,
    },
    KeyDefinition {
        name: "KeyU",
        code: Key::KeyU,
    },
    KeyDefinition {
        name: "KeyV",
        code: Key::KeyV,
    },
    KeyDefinition {
        name: "KeyW",
        code: Key::KeyW,
    },
    KeyDefinition {
        name: "KeyX",
        code: Key::KeyX,
    },
    KeyDefinition {
        name: "KeyY",
        code: Key::KeyY,
    },
    KeyDefinition {
        name: "KeyZ",
        code: Key::KeyZ,
    },
    KeyDefinition {
        name: "Num1",
        code: Key::Num1,
    },
    KeyDefinition {
        name: "Num2",
        code: Key::Num2,
    },
    KeyDefinition {
        name: "Num3",
        code: Key::Num3,
    },
    KeyDefinition {
        name: "Num4",
        code: Key::Num4,
    },
    KeyDefinition {
        name: "Num5",
        code: Key::Num5,
    },
    KeyDefinition {
        name: "Num6",
        code: Key::Num6,
    },
    KeyDefinition {
        name: "Num7",
        code: Key::Num7,
    },
    KeyDefinition {
        name: "Num8",
        code: Key::Num8,
    },
    KeyDefinition {
        name: "Num9",
        code: Key::Num9,
    },
    KeyDefinition {
        name: "F1",
        code: Key::F1,
    },
    KeyDefinition {
        name: "F2",
        code: Key::F2,
    },
    KeyDefinition {
        name: "F3",
        code: Key::F3,
    },
    KeyDefinition {
        name: "F4",
        code: Key::F4,
    },
    KeyDefinition {
        name: "F5",
        code: Key::F5,
    },
    KeyDefinition {
        name: "F6",
        code: Key::F6,
    },
    KeyDefinition {
        name: "F7",
        code: Key::F7,
    },
    KeyDefinition {
        name: "F8",
        code: Key::F8,
    },
    KeyDefinition {
        name: "F9",
        code: Key::F9,
    },
    KeyDefinition {
        name: "F10",
        code: Key::F10,
    },
    KeyDefinition {
        name: "F11",
        code: Key::F11,
    },
    KeyDefinition {
        name: "F12",
        code: Key::F12,
    },
    KeyDefinition {
        name: "Space",
        code: Key::Space,
    },
    KeyDefinition {
        name: "Enter",
        code: Key::Return,
    },
    KeyDefinition {
        name: "Escape",
        code: Key::Escape,
    },
    KeyDefinition {
        name: "Tab",
        code: Key::Tab,
    },
    KeyDefinition {
        name: "Backspace",
        code: Key::Backspace,
    },
    KeyDefinition {
        name: "ArrowUp",
        code: Key::UpArrow,
    },
    KeyDefinition {
        name: "ArrowDown",
        code: Key::DownArrow,
    },
    KeyDefinition {
        name: "ArrowLeft",
        code: Key::LeftArrow,
    },
    KeyDefinition {
        name: "ArrowRight",
        code: Key::RightArrow,
    },
    KeyDefinition {
        name: "ControlLeft",
        code: Key::ControlLeft,
    },
    KeyDefinition {
        name: "ControlRight",
        code: Key::ControlRight,
    },
    KeyDefinition {
        name: "ShiftLeft",
        code: Key::ShiftLeft,
    },
    KeyDefinition {
        name: "ShiftRight",
        code: Key::ShiftRight,
    },
    KeyDefinition {
        name: "AltLeft",
        code: Key::Alt,
    },
    KeyDefinition {
        name: "AltRight",
        code: Key::AltGr,
    },
    KeyDefinition {
        name: "SuperLeft",
        code: Key::MetaLeft,
    },
    KeyDefinition {
        name: "SuperRight",
        code: Key::MetaRight,
    },
    KeyDefinition {
        name: "WinLeft",
        code: Key::MetaLeft,
    },
    KeyDefinition {
        name: "WinRight",
        code: Key::MetaRight,
    },
];

pub fn parse_key_code(name: &str) -> Result<Key, String> {
    KEY_DEFINITIONS
        .iter()
        .find(|definition| definition.name == name)
        .map(|definition| definition.code)
        .ok_or_else(|| format!("unknown key code: {name}"))
}

pub fn deserialize_key_bindings<'de, D>(deserializer: D) -> Result<Vec<Key>, D::Error>
where
    D: Deserializer<'de>,
{
    let names = Vec::<String>::deserialize(deserializer)?;
    names
        .into_iter()
        .map(|name| parse_key_code(&name).map_err(serde::de::Error::custom))
        .collect()
}

pub fn digit_of(key: Key) -> Option<u32> {
    match key {
        Key::Num1 => Some(1),
        Key::Num2 => Some(2),
        Key::Num3 => Some(3),
        Key::Num4 => Some(4),
        Key::Num5 => Some(5),
        Key::Num6 => Some(6),
        Key::Num7 => Some(7),
        Key::Num8 => Some(8),
        Key::Num9 => Some(9),
        _ => None,
    }
}

pub struct Hotkey {
    pub(crate) keys: HashSet<Key>,
    pub(crate) trigger: Key,
    pub(crate) name: &'static str,
    pub(crate) callback: Box<dyn Fn() + Send>,
    pub(crate) triggered: bool,
    pub(crate) exact: bool,
}

const DIGIT_KEYS: [Key; 9] = [
    Key::Num1,
    Key::Num2,
    Key::Num3,
    Key::Num4,
    Key::Num5,
    Key::Num6,
    Key::Num7,
    Key::Num8,
    Key::Num9,
];

pub fn build_hotkeys(feature: &ActiveWindowFeature) -> Vec<Hotkey> {
    let modifiers = &feature.config.key_bindings;
    match feature.function {
        FeatureAction::Plain(function) => {
            let Some(&trigger) = modifiers.last() else {
                return Vec::new();
            };
            vec![Hotkey {
                keys: modifiers.iter().copied().collect(),
                trigger,
                name: feature.name,
                callback: Box::new(function),
                triggered: false,
                exact: false,
            }]
        }
        FeatureAction::Drag => Vec::new(),
        FeatureAction::Numbered(function) => DIGIT_KEYS
            .into_iter()
            .filter_map(|trigger| {
                let number = digit_of(trigger)?;
                Some(Hotkey {
                    keys: modifiers.iter().copied().chain([trigger]).collect(),
                    trigger,
                    name: feature.name,
                    callback: Box::new(move || function(number)),
                    triggered: false,
                    exact: true,
                })
            })
            .collect(),
    }
}
