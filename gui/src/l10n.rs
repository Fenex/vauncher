use std::{collections::HashMap, sync::Arc};

use druid::{ArcStr, Env, Key, LocalizedString};

#[derive(Debug, Clone)]
pub struct Localization {
    cache: HashMap<&'static str, ArcStr>,
}

pub const L10N: Key<Arc<Localization>> = Key::new("vauncher.localization");

const L10N_KEYS: &[&str] = &[
    "nav-title-main",
    "nav-title-versions",
    "footer-button-exit",
    "footer-button-play",
];

impl Localization {
    pub fn new(env: &Env) -> Self {
        let mut this = Self {
            cache: HashMap::new(),
        };
        this.update(env);
        this
    }

    pub fn update(&mut self, env: &Env) {
        self.cache.clear();

        for &key in L10N_KEYS {
            let mut l10n = LocalizedString::new(key);
            l10n.resolve(&(), env);
            self.cache.insert(key, l10n.localized_str());
        }
    }
}

impl Localization {
    pub fn get_str<'s: 'out, 'k: 'out, 'out>(&'s self, key: &'k str) -> &'out str {
        self.cache.get(key).map(|value| &**value).unwrap_or(key)
    }

    pub fn get(&self, key: &str) -> String {
        self.get_str(key).to_owned()
    }
}
