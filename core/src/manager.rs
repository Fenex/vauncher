use std::{
    borrow::Cow,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagerCfg {
    #[serde(skip)]
    path: PathBuf,
    // lang: String,
    root: Option<String>,
}

impl ManagerCfg {
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(reader)
    }
}

#[derive(Debug)]
pub struct Manager2 {
    config: ManagerCfg,
    mods: Vec<()>, /*modification::Record>*/
}

impl Manager2 {
    pub const DEFAULT_MODS_FOLDER: &'static str = "mods";

    /// Path to directory that will be storage mods
    pub fn root(&self) -> Cow<Path> {
        self.config
            .root
            .as_ref()
            .map(|root| Path::new(root).into())
            .unwrap_or_else(|| {
                std::env::current_dir()
                    .unwrap()
                    .join(Self::DEFAULT_MODS_FOLDER)
                    .into()
            })
    }

    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let mut this = Self::from_file(path).unwrap_or_else(|_| {
            Manager2 {
                config: ManagerCfg {
                    path: std::env::current_dir().unwrap(),
                    // lang: String::from(""),
                    root: None,
                },
                mods: vec![],
            }
        });

        this.update();
        this
    }

    pub fn update(&mut self) -> Option<bool> {
        for entry in self.root().read_dir().ok()? {
            let entry = {
                if entry.is_err() {
                    continue;
                }
                entry.unwrap()
            };

            let metadata = {
                let meta = entry.metadata();
                if meta.is_err() {
                    continue;
                }
                meta.unwrap()
            };

            if metadata.is_dir() {
                continue;
                // todo!("directories");
            }

            if metadata.is_file() {
                // insert newer and update older,
                // todo: remove from `Vec` if file(s) was deleted
                println!("file found: {}", PathBuf::from(entry.file_name()).display());
                let p = entry.path();
                // if let Some(record) = modification::Record::from_file(&p) {
                //     match self.mods.iter_mut().find(|m| m.config == record.config) {
                //         Some(vmod) => *vmod = record,
                //         None => self.mods.push(record)
                //     }
                // }
            }
        }

        Some(true)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let mut config = ManagerCfg::from_reader(&mut reader)?;
        config.path = path.as_ref().to_owned();

        Ok(Self {
            config,
            mods: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let manager = Manager2::new("./tests/vm/vauncher_testdir.json");
        // assert_eq!(manager.config.lang, String::from(""));
        // assert_eq!(manager.root().to_str(), Some(".."));
        let s = serde_json::ser::to_string(&manager.config);
        println!("{:?}", &manager.root());
        println!("mods: {:?}", &manager.mods);
        assert!(s.is_ok());
    }
}
