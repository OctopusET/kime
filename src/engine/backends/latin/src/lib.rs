use kime_engine_backend::{AHashMap, InputEngineBackend, Key};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum LatinLayout {
    Qwerty,
    Colemak,
}

#[derive(Serialize, Deserialize)]
pub struct LatinConfig {
    layout: LatinLayout,
}

impl Default for LatinConfig {
    fn default() -> Self {
        Self {
            layout: LatinLayout::Qwerty,
        }
    }
}

#[derive(Clone)]
pub struct LatinEngine {
    layout: AHashMap<Key, char>,
}

impl LatinEngine {
    pub fn new(config: &LatinConfig) -> Self {
        let layout = match config.layout {
            LatinLayout::Qwerty => include_str!("../data/qwerty.yaml"),
            LatinLayout::Colemak => include_str!("../data/colemak.yaml"),
        };
        Self {
            layout: serde_yaml::from_str(layout).unwrap_or_default(),
        }
    }
}

impl InputEngineBackend for LatinEngine {
    fn press_key(&mut self, key: Key, commit_buf: &mut String) -> bool {
        if let Some(ch) = self.layout.get(&key) {
            commit_buf.push(*ch);
            true
        } else {
            false
        }
    }

    fn clear_preedit(&mut self, _commit_buf: &mut String) {}
    fn reset(&mut self) {}

    fn has_preedit(&self) -> bool {
        false
    }

    fn preedit_str(&self, _buf: &mut String) {}
}
