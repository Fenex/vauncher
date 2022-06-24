use druid::{widget::*, Data, Env, Event, Lens, LensExt, Selector, Widget};
mod about;
mod main;
mod mods;
mod versions;

use crate::nav::TabId;
use crate::AppState;

pub fn body() -> impl Widget<AppState> {
    Scroll::new(ViewSwitcher::new(
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
    ))
    .expand()
}

#[derive(Debug, Default, Clone, Data, Lens)]
pub struct CardItemBase {
    name: String,
    path_bin: String,
    path_resources: String,
}

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
