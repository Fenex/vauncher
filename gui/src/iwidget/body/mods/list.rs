use crate::{druid::*, vcore::ModificationConfigD};

use super::{empty::template as empty, *};

pub(super) fn template() -> impl Widget<ModsState> {
    Flex::column()
    .with_child(header("Modifications / Overview"))
    .with_flex_child(Either::new(
        |d: &ModsState, _| d.items.is_empty(),
        empty().lens(Constant(())),
        list(),
    ), 1.)
    .expand()

}

fn list() -> impl Widget<ModsState> {
    Scroll::new(
        List::new(|| {
            item()
        })
        .lens(ModsState::items)
        .controller(ListModsController)
    )
    .vertical()
}

fn item() -> impl Widget<ModificationConfigD> {
    Flex::row()
        .with_flex_child(Label::dynamic(|x: &ModificationConfigD, _env| x.0.package.as_ref().map(|p| p.name.to_string()).unwrap_or("[UNDEFINED_NAME]".to_string())).expand_width(), 1.0)
        .with_child(Button::new("X"))
        .border(Color::YELLOW, 1.0)
        .rounded(4.)
}

struct ListModsController;

impl<W: Widget<ModsState>> Controller<ModsState, W> for ListModsController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut ModsState,
        env: &Env,
    ) {
        child.event(ctx, event, data, env)
    }
}
