use std::time::SystemTime;

use druid::{widget::*, Data, Env, Event, Lens, LensExt, Selector, Widget};

mod about;
mod main;
mod mods;
mod versions;

use crate::nav::TabId;
use crate::AppState;

pub fn body() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| data.variant(),
        |selector, _data, _env| {
            match *selector {
                main::TAB_KEY => main::template().boxed(),
                versions::TAB_KEY => versions::template().boxed(),
                mods::TAB_KEY => mods::template().boxed(),
                about::TAB_KEY => about::template().boxed(),
            }
            .padding(5.)
            .boxed()
        },
    )
    .expand()
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
    is_hover: bool
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
            is_hover: false
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.record.name = name;
        self
    }
}

// pub struct CardItemBase2CardItemEditable;

// impl<'a> Lens<CardItemBase, CardItemEditable> for CardItemBase2CardItemEditable {
//     fn with<V, F: FnOnce(&CardItemEditable) -> V>(&self, data: &CardItemBase, f: F) -> V {
//         f(&CardItemEditable {
//             id: data.id,
//             original: data,
//             edit: data.clone()
//         })
//     }

//     fn with_mut<V, F: FnOnce(&mut CardItemEditable) -> V>(&self, data: &mut CardItemBase, f: F) -> V {
//         f(&mut CardItemEditable {
//             id: data.id,
//             original: data.clone(),
//             edit: data.clone()
//         })
//     }
// }

pub trait BodySwitcher {
    type Variant: Data;
    fn variant(&self) -> Self::Variant;
}

impl BodySwitcher for AppState {
    type Variant = TabId;
    fn variant(&self) -> Self::Variant {
        self.tab
    }
}
