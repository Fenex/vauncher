use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    time::SystemTime,
};

use crate::{druid::*, vcore::ModificationConfigD};

use vcore::ModificationConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Data)]
pub enum TabModsId {
    List,
    Add,
}

impl Default for TabModsId {
    fn default() -> Self {
        Self::List
    }
}

pub trait BodySwitcher {
    type Variant: Data;
    fn variant(&self) -> Self::Variant;
}

impl BodySwitcher for ModsState {
    type Variant = TabModsId;
    fn variant(&self) -> Self::Variant {
        self.tab
    }
}

#[derive(Debug, Default, Clone, Data, Lens)]
pub struct ModsState {
    pub items: im::Vector<ModificationConfigD>,
    pub tab: TabModsId,
}

#[derive(Debug, Default, Clone, Data)]
pub struct ModItem {}

pub trait IModItem {
    type Version: ?Sized;

    /// Returns name of the mod
    fn name(&self) -> &str;

    /// Returns description of the mod
    fn description(&self) -> Option<&str>;

    /// Returns authors of the mod if exists
    fn authors(&self) -> Option<&str>;

    /// Returns installed version of the mod
    /// `None` will be returned if this mod is not installed
    fn version(&self) -> &Self::Version;

    /// Returns list of available versions for installing
    fn version_list(&self) -> &[&Self::Version];

    /// Make given version as `fixed` (disable autoupdate)
    /// `Err` will be returned if given version is not exists
    fn set_fixed_version(&mut self, version: &Self::Version) -> Result<(), ()>;

    /// Returns version if fixed (disable auto-updating)
    fn get_fixed_version(&self) -> Option<&Self::Version>;

    /// Reset current version as fixed
    fn reset_fixed_version(&mut self);

    /// Install last version
    fn update(&mut self) -> Result<(), ()>;

    /// Returns path to entry point (usually `main.js`) in local file system
    fn entry_path(&self) -> Option<&str>;

    /// Returns status of the mod;
    fn is_enabled(&self) -> bool;

    /// enable this mod to use
    fn enable(&mut self) -> Result<(), ()>;

    /// disable this mod to use
    fn disable(&mut self) -> Result<(), ()>;

    /// Returns `true` if mod is installed
    fn is_installed(&self) -> bool {
        unimplemented!()
        // self.version().is_some()
    }

    fn is_disabled(&self) -> bool {
        !self.is_enabled()
    }
}

// #[derive(Debug, Clone, Data)]
pub struct Local {
    // #[data(eq)]
    pub metafile: PathBuf,
    meta: json::Root,
    is_enabled: bool,
}

mod json {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Vauncher {
        pub entry: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Root {
        pub name: String,
        pub version: String,
        pub description: Option<String>,
        pub author: Option<String>,
        pub vauncher: Option<Vauncher>,
    }
}

impl Local {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let meta = serde_json::from_reader(reader)?;

        let this = Self {
            metafile: path.as_ref().to_owned(),
            meta,
            is_enabled: true,
        };

        Ok(this)
    }
}

impl Local {
    const VERSION: &'static str = "local";
}

impl IModItem for Local {
    type Version = str;

    fn name(&self) -> &str {
        &self.meta.name
    }

    fn description(&self) -> Option<&str> {
        self.meta.description.as_deref()
    }

    fn authors(&self) -> Option<&str> {
        self.meta.author.as_deref()
    }

    fn version(&self) -> &Self::Version {
        &self.meta.version
    }

    fn version_list(&self) -> &[&Self::Version] {
        &[Self::VERSION]
    }

    fn set_fixed_version(&mut self, _version: &Self::Version) -> Result<(), ()> {
        Err(())
    }

    fn get_fixed_version(&self) -> Option<&Self::Version> {
        Some(Self::VERSION)
    }

    fn reset_fixed_version(&mut self) {}

    fn update(&mut self) -> Result<(), ()> {
        Err(())
    }

    fn entry_path(&self) -> Option<&str> {
        self.metafile.as_os_str().to_str()
    }

    fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    fn enable(&mut self) -> Result<(), ()> {
        self.is_enabled = true;
        Ok(())
    }

    fn disable(&mut self) -> Result<(), ()> {
        self.is_enabled = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_from_path() {
        let local = Local::from_path("./core/tests/vm/mods/mod_name.json");

        assert!(local.is_ok());
    }
}

fn generate_uid() -> String {
    let bytes = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|e| e.duration())
        .as_nanos()
        .to_le_bytes();

    format!("{:?}", Md5::digest(bytes))
}
