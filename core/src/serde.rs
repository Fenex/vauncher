use std::{fs::{DirEntry, File}, io::BufReader, path::Path};

use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json::{json, Value};

use self::modcfg::{Local, Github};

#[derive(Serialize, Deserialize)]
pub struct Modcfg {
    source: Source,
    disabled: bool,
}

pub enum Source {
    Local(Local),
    Github(Github),
    // GzByURL
}

// pub trait ModSource {
//     fn r#type(&self) -> Source;

// }


// pub trait ModCfgItem {
//     fn r#type(&self) -> &str;
// }

// pub mod modcfg {
//     use super::*;

//     #[derive(Debug, Serialize, Deserialize)]
//     pub struct Local(String);

//     impl ModCfgItem for Local {
//         fn r#type(&self) -> &str {
//             "local"
//         }
//     }

//     #[derive(Debug, Serialize, Deserialize)]
//     pub struct Github {
//         owner: String,
//         repo: String,
//     }

//     impl ModCfgItem for Github {
//         fn r#type(&self) -> &str {
//             "github"
//         }
//     }
// }

// impl Modcfg<modcfg::Local> {
//     // pub fn from_entry(entry: &DirEntry) -> Result<Self, Box<dyn std::error::Error>> {
//     //     let path = entry.path();
//     //     // Self {

//     //     // }
//     // }
// }


// #[derive(Serialize, Deserialize)]
// struct ModType {
//     r#type: String
// }

// impl<T: DeserializeOwned> Modcfg<T> {
//     pub fn from_file<P: AsRef<Path>>(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
//         let file = File::open(path)?;
//         let reader = BufReader::new(file);
//         let json: Value = serde_json::from_reader(reader)?;
//         let this = match json.get("type") {
//             Some(v) if v.is_string() => {
//                 match v.as_str().unwrap() {
//                     "local" => Modcfg::<Local>::from(v.clone()),
//                     "github" => Modcfg::<Github>::from(v.clone()),
//                     None => unreachable!(),
//                 }
//             }
//             _ => unreachable!(),
//         };

//         Ok(this)
//     }
// }

// #[derive(Serialize, Deserialize)]
// pub struct Package {
//     name: String,
//     version: String,
//     author: Option<String>,
// }
