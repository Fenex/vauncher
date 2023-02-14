use std::{env, fs::File, io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::druid::Data;

#[derive(Debug, Data, Clone, Serialize, Deserialize)]
pub struct AppPrefs {
    #[serde(skip)]
    #[data(ignore)]
    path_to_this_file: Option<PathBuf>,
    #[data(eq)]
    pub root: PathBuf,
}

impl Default for AppPrefs {
    fn default() -> Self {
        Self {
            root: PathBuf::from("."),
            path_to_this_file: None,
        }
    }
}

impl AppPrefs {
    const FILE: &str = "prefs.json";

    pub fn init() -> Self {
        let paths = &[
            env::current_dir().ok(),
            env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(ToOwned::to_owned)),
        ];

        for path in paths.into_iter().filter_map(ToOwned::to_owned) {
            let file = path.join(Self::FILE);
            if let Ok(file) = File::open(file) {
                if let Ok(mut prefs) = serde_json::from_reader::<_, AppPrefs>(BufReader::new(file))
                {
                    if !prefs.root.is_absolute() {
                        prefs.root = path.join(prefs.root);
                    }
                    prefs.path_to_this_file = Some(path);
                    return prefs;
                }
            }
        }

        Default::default()
    }
}
