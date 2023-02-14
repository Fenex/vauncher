mod _state;
mod add;
mod empty;
mod list;

use self::_state::BodySwitcher;

use super::*;

pub use _state::*;
use druid::{lens::Constant, EventCtx};
use crate::vcore::ModificationConfig;

pub(in self) fn header<T: Data>(text: impl Into<LabelText<T>>) -> impl Widget<T> {
    Flex::column()
        .with_child(
            Label::new(text)
                .with_text_size(22.)
                .with_text_color(Color::GRAY)
                .expand_width(),
        )
        .with_child(
            SizedBox::empty()
                .background(Color::YELLOW)
                .expand_width()
                .fix_height(1.)
                .padding((0., 10.)),
        )
}

pub(super) const TAB_KEY: TabId = TabId::Mods;

pub(super) fn template() -> impl Widget<ModsState> {
    ViewSwitcher::new(
        |data: &ModsState, _env| data.variant(),
        |selector, _data, _env| {
            match *selector {
                TabModsId::Add => add::template().lens(Constant(())).boxed(),
                TabModsId::List => list::template().boxed(),
            }
            .padding(5.)
            .boxed()
        },
    )
    .expand()
    .controller(ModsPageController)
}

const CHANGE_TAB_MOD_ID: Selector<TabModsId> = Selector::new("change mod page");
pub const ADD_MOD: Selector<ModificationConfig> = Selector::new("add new mod into overview list");

struct ModsPageController;

impl<W: Widget<ModsState>> Controller<ModsState, W> for ModsPageController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut ModsState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(CHANGE_TAB_MOD_ID) => {
                let tab_id = *cmd.get_unchecked(CHANGE_TAB_MOD_ID);
                data.tab = tab_id;
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(ADD_MOD) => {
                let config = cmd.get_unchecked(ADD_MOD).clone();
                data.items.push_back(config.into());
                data.tab = TabModsId::List;
                ctx.set_handled();
            }
            _ => (),
        }
        child.event(ctx, event, data, env);
    }
}
