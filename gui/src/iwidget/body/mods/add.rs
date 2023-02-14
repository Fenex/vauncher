mod _state;
use _state::*;

mod github;
use github::*;

mod local;
use local::*;

use crate::druid::*;
use druid_widget_nursery::DropdownSelect;

use super::header;

pub(super) fn template<T: Data>() -> impl Widget<T> {
    Flex::column()
        .with_child(header("Modifications / Add"))
        .with_flex_child(body().expand(), 1.)
        .expand()
    // .debug_paint_layout()
}

fn body<T: Data>() -> impl Widget<T> {
    Scope::new(
        PageState::scope(),
        Flex::column()
            .with_child(
                Label::new(|_: &PageState, _: &druid::Env| {
                    (0..40)
                        .fold(vec![], |mut acc, _| {
                            acc.push("MODS_DESCRIPTION LONG");
                            acc
                        })
                        .join(" ")
                })
                .with_line_break_mode(LineBreaking::WordWrap)
                .with_text_color(Color::GRAY)
                .expand_width(),
            )
            .with_child(dl_row(
                "SOURCE_TYPE:",
                DropdownSelect::new(vec![
                    ("local", ModSource::Local),
                    ("github", ModSource::Github),
                ])
                // .fix_height(50.)
                .expand_width()
                .align_left()
                .lens(PageState::source),
            ))
            .with_child(ViewSwitcher::new(
                |data: &PageState, _env| data.source,
                |selector, _data, _env| {
                    match *selector {
                        ModSource::Local => local::template().lens(PageState::local).boxed(),
                        ModSource::Github => github::template().lens(PageState::github).boxed(),
                    }
                    .boxed()
                },
            )),
    )
    .debug_paint_layout()
    .lens(Unit)
}

fn dl_row<T: Data, W: Widget<T> + 'static, L: Into<LabelText<T>> + 'static>(
    text: L,
    widget: W,
) -> impl Widget<T> {
    Flex::row()
        .with_child(
            Container::new(Label::new(text).align_right())
                .fix_width(200.)
                .padding((20., 10.)),
        )
        .with_flex_child(widget, 1.)
}

#[derive(Default, Debug, Data, Clone, Copy, PartialEq, Eq)]
pub enum ModSource {
    #[default]
    Local,
    Github,
}

// struct EmptyModsController;

// impl<W: Widget<()>> Controller<(), W> for EmptyModsController {
//     fn event(
//         &mut self,
//         child: &mut W,
//         ctx: &mut druid::EventCtx,
//         event: &druid::Event,
//         data: &mut (),
//         env: &druid::Env,
//     ) {
//         child.event(ctx, event, data, env)
//     }
// }
