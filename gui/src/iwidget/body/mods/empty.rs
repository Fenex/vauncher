use crate::druid::*;
use ::druid::text::RichTextBuilder;

use super::*;

pub(super) fn template() -> impl Widget<()> {
    let mut builder = RichTextBuilder::new();
    builder
        .push("Добавьте")
        .underline(true)
        .text_color(Color::rgb8(50, 50, 160))
        .link(CHANGE_TAB_MOD_ID.with(TabModsId::Add));
    builder.push(" мод в список");

    let text = RawLabel::new()
        .with_text_size(40.)
        .with_text_color(Color::GRAY)
        .with_line_break_mode(LineBreaking::WordWrap)
        .lens(Constant(builder.build()));

    Flex::column()
        .with_child(text)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .expand()
}
