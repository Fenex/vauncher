use std::{
    borrow::Cow,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use rand::random;
use serde::{Deserialize, Serialize};

mod github;
mod local;

pub(super) mod prelude {

    pub use super::github::Github as ModificationSourceGithub;
    pub use super::local::Local as ModificationSourceLocal;
    pub use super::Config as ModificationConfig;
    pub use super::Package as ModificationPackage;
    pub use super::Source as ModificationSource;
}

use prelude::*;

/// Source type of the mod and its uniq ID
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    Github(ModificationSourceGithub),
    Local(ModificationSourceLocal),
    // Git(Git)
}

/// json-файл, в котором располагаются общие данные о моде и его настройках;
/// создаваться и управляться должен исключительно этим крейтом программно
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    // #[serde(skip)]
    // pub path: Option<PathBuf>,
    /// некоторые поля из package.json мода
    pub package: Option<Package>,
    /// тип источника для мода + уникальный идентификатор в этом источнике
    pub source: Source,
}

impl Config {
    // pub fn name(&self) -> &str {
    //     // TODO: get the name by request to `Source` instead of `self.path`
    //     self.package
    //         .as_ref()
    //         .map(|p| p.name.as_str())
    //         .unwrap_or_else(|| self.path.file_name().and_then(|a| a.to_str()).unwrap())
    // }

    /// Указывается, доступен ли мод.
    /// В теории может быть недоступен, если файл(ы) мода были удалены,
    /// а конфиг - нет.
    pub fn is_available(&self) -> bool {
        self.package.is_some()
    }
}

impl Config {
    pub fn new(source: Source) -> Option<Self> {
        let package = match source {
            Source::Github(_) => unimplemented!(),
            Source::Local(ref local) => {
                let path = Path::new(&local.0);

                let file = File::open(path).unwrap(); // TODO: unwrap
                let mut reader = BufReader::new(file);
                serde_json::from_reader::<_, Package>(&mut reader).ok() //TODO: error
            }
        };

        Some(Self { source, package })
    }

    pub fn save<P: AsRef<Path>>(&self, file: P) {
        let file = File::create(file.as_ref()).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self).unwrap();
    }

    // TODO: use `Result` instead of `Option`
    // TODO: refactor: diff alghorithms in various source types
    // pub fn fetch<P: AsRef<Path>>(source: Source, mods_dir: P) -> Option<Self> {
    //     let dist = mods_dir.as_ref();
    //     let package = match source {
    //         Source::Github(_) => {
    //             unimplemented!();
    //         }
    //         Source::Local(ref local) => {
    //             let path = Path::new(&local.0);

    //             if matches!(path.file_name(), Some(fname) if fname == Package::FILE_NAME) {
    //                 let file = File::open(path).unwrap(); // TODO: unwrap
    //                 let mut reader = BufReader::new(file);
    //                 serde_json::from_reader::<_, Package>(&mut reader).ok()? //TODO: error
    //             } else {
    //                 None?
    //             }
    //         }
    //     };

    //     let file = 'external: loop {
    //         const GENERATION_TRIES: u8 = 10;
    //         for _ in 0..GENERATION_TRIES {
    //             let file_name = format!("mod_{:05}.json", random::<u16>());
    //             let dist_file = dist.join(file_name);
    //             if !dist_file.exists() {
    //                 break 'external dist_file;
    //             }
    //         }

    //         return None; // N generated names was taken already
    //     };

    //     let this = Self {
    //         path: file,
    //         package: Some(package),
    //         source,
    //     };

    //     let file = File::create(&this.path).ok()?;
    //     let writer = BufWriter::new(file);
    //     serde_json::to_writer_pretty(writer, &this).ok()?;

    //     Some(this)
    // }

    // pub fn load<P: AsRef<Path>>(config: P) -> Option<Self> {
    //     let file = File::open(&config).ok()?;
    //     let mut reader = BufReader::new(file);
    //     let mut this = serde_json::from_reader::<_, Self>(&mut reader).ok()?;

    //     this.path = config.as_ref().to_owned();

    //     Some(this)
    // }
}

/// Некоторые поля из модов (package.json)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Package {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    /// точка входа мода (обычно main.js)
    pub vauncher: Option<String>,
}

impl Package {
    const FILE_NAME: &str = "package.json";
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn config_fetch() {
    //     let config = Config::fetch(
    //         Source::Local(Local(
    //             "./tests/vm/mods/mod_1/1.0.0/package.json".to_string(),
    //         )),
    //         "./tests/vm/mods/",
    //     );
    // }

    // #[test]
    // fn config_load() {
    //     let config = Config::load("./tests/vm/mods/mod_name.json");
    //     dbg!(config);
    // }
}

// mod types {
//     use super::*;

//     #[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
//     #[serde(rename_all = "snake_case")]
//     pub enum ModType {
//         #[default]
//         None,
//         Local(Local),
//         Github(Github),
//     }

//     #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
//     pub struct Local(String);

//     #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
//     pub struct Github {
//         owner: String,
//         repo: String,
//     }

// }

// use types::*;

// #[derive(Debug)]
// pub struct Record {
//     pub config: Config,
// }

// impl Record {
//     pub fn from_file<P: AsRef<Path>>(path: P) -> Option<Self> {
//         Config::from_file(path).map(|config| Self { config })
//     }
// }
