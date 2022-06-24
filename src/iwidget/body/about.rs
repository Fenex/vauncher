use druid::{widget::*, *};

use super::*;

pub(super) const TAB_KEY: TabId = TabId::About;

pub(super) fn template<T: Data>() -> impl Widget<T> {
    Flex::column()
        .with_child(Label::new("Vauncher: Vangers Launcher").with_text_size(20.))
        .with_child(Label::new("© Vitaliy Busko and Vangers Community, 2022"))
}
