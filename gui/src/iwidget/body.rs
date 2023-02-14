use crate::druid::*;

mod about;
mod main;
mod mods;
mod versions;

use super::*;

pub use mods::*;

use crate::AppState;

pub fn template() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| data.variant(),
        |selector, _data, _env| {
            match *selector {
                main::TAB_KEY => main::template().boxed(),
                versions::TAB_KEY => versions::template().boxed(),
                mods::TAB_KEY => mods::template().lens(AppState::mods).boxed(),
                about::TAB_KEY => about::template().boxed(),
            }
            .padding(5.)
            .boxed()
        },
    )
    .expand()
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
