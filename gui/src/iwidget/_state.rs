use std::{time::SystemTime, sync::Arc};

use vcore::Manager2;

use crate::druid::*;

use super::*;
use crate::prefs::AppPrefs;

#[derive(Debug, Clone, Data, Lens)]
pub struct AppState {
    pub prefs: AppPrefs,
    pub tab: TabId,
    pub cards: im::Vector<CardItem>,
    pub mods: ModsState,
    // #[data(ignore)]
    // pub mods_manager: Arc<Manager2>,
}

#[derive(Debug, Default, Clone, Data, Lens)]
pub struct CardItemRecord {
    pub name: String,
    pub path_bin: String,
    pub path_resources: String,
}

#[derive(Debug, Clone, Data)]
pub struct CardItem {
    pub id: u32,
    pub record: CardItemRecord,
    pub edit: Option<CardItemRecord>,
    pub(super) is_hover: bool,
}

impl CardItem {
    pub fn new() -> Self {
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|r| r.duration())
            .as_millis() as u32;

        Self {
            id: ts,
            record: Default::default(),
            edit: Default::default(),
            is_hover: false,
        }
    }

    pub fn make_edit(&mut self) {
        self.edit = Some(self.record.clone());
    }

    pub fn with_edit(mut self) -> Self {
        self.make_edit();
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.record.name = name;
        self
    }
}

impl Default for CardItem {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CardItemRead;

impl Lens<CardItem, CardItemRecord> for CardItemRead {
    fn with<V, F: FnOnce(&CardItemRecord) -> V>(&self, data: &CardItem, f: F) -> V {
        f(&data.record)
    }

    fn with_mut<V, F: FnOnce(&mut CardItemRecord) -> V>(&self, data: &mut CardItem, f: F) -> V {
        f(&mut data.record)
    }
}

pub struct CardItemEdit;

impl Lens<CardItem, CardItemRecord> for CardItemEdit {
    fn with<V, F: FnOnce(&CardItemRecord) -> V>(&self, data: &CardItem, f: F) -> V {
        f(data.edit.as_ref().unwrap())
    }

    fn with_mut<V, F: FnOnce(&mut CardItemRecord) -> V>(&self, data: &mut CardItem, f: F) -> V {
        f(data.edit.as_mut().unwrap())
    }
}
