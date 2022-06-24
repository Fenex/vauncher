use super::*;

pub(super) const TAB_KEY: TabId = TabId::Mods;

pub(super) fn template<T: Data>() -> impl Widget<T> {
    Flex::column().with_child(Label::new("Mods tab").with_text_size(20.))
}
