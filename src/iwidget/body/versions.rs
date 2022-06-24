use super::*;

pub(super) const TAB_KEY: TabId = TabId::Versions;

pub(super) fn template<T: Data>() -> impl Widget<T> {
    Flex::column().with_child(Label::new("Versions tab").with_text_size(20.))
}
