use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Data)]
pub enum TabId {
    Main,
    Versions,
    Mods,
    About,
}

impl Default for TabId {
    fn default() -> Self {
        Self::Main
    }
}

pub fn nav() -> impl Widget<AppState> {
    Container::new(
        Flex::column()
            .with_child(
                link("nav-title-main")
                    .controller(NavControllerItem::default())
                    .lens(AppState2NavItemState { this: TabId::Main }),
            )
            .with_child(
                link("Versions")
                    .controller(NavControllerItem::default())
                    .lens(AppState2NavItemState {
                        this: TabId::Versions,
                    }),
            )
            .with_child(
                link("Mods")
                    .controller(NavControllerItem::default())
                    .lens(AppState2NavItemState { this: TabId::Mods }),
            ),
    )
    .fix_width(200.)
    .expand_height()
    .controller(NavController)
}

fn link(text_key: &'static str) -> impl Widget<NavItemState> {
    let paint = Painter::new(|ctx, data: &NavItemState, _env| {
        let brush = if data.is_active {
            Color::rgb8(41, 41, 41)
        } else {
            Color::TRANSPARENT
        };
        let rect = ctx.region().bounding_box();
        ctx.fill(rect, &brush);
    });

    Label::dynamic(|_data, env| env.get(L10N).get(text_key))
        .with_text_size(22.)
        .padding((15., 5.))
        .expand_width()
        .background(paint)
}

struct NavController;

impl<W: Widget<AppState>> Controller<AppState, W> for NavController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Notification(p) if p.is(NAV_CLICKED) => {
                data.tab = *p.get(NAV_CLICKED).unwrap();
                ctx.set_handled();
            }
            _ => (),
        }
        child.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &AppState,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        child.update(ctx, old_data, data, env)
    }
}

const NAV_CLICKED: Selector<TabId> = Selector::new("navigator btn clicked");

#[derive(Debug, Default)]
struct NavControllerItem {
    is_hover: bool,
}

impl<W: Widget<NavItemState>> Controller<NavItemState, W> for NavControllerItem {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut NavItemState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(HOT_CHANGED) => {
                self.is_hover = *cmd.get(HOT_CHANGED).unwrap();
                if self.is_hover {
                    ctx.override_cursor(&Cursor::Pointer);
                } else {
                    ctx.clear_cursor();
                }
                ctx.set_handled();
            }
            Event::MouseUp(_) => ctx.submit_notification(NAV_CLICKED.with(data.this)),
            _ => (),
        }

        child.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &NavItemState,
        env: &Env,
    ) {
        match event {
            LifeCycle::HotChanged(e) => {
                ctx.submit_command(HOT_CHANGED.with(*e).to(Target::Widget(ctx.widget_id())));
            },
            _ => ()
        }
        child.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, child: &mut W, ctx: &mut UpdateCtx, old_data: &NavItemState, data: &NavItemState, env: &Env) {
        child.update(ctx, old_data, data, env)
    }
}

#[derive(Debug, Clone, Data)]
struct NavItemState {
    this: TabId,
    is_active: bool,
    is_hover: bool,
}

struct AppState2NavItemState {
    this: TabId,
}

impl Lens<AppState, NavItemState> for AppState2NavItemState {
    fn with<V, F: FnOnce(&NavItemState) -> V>(&self, data: &AppState, f: F) -> V {
        f(&NavItemState {
            this: self.this,
            is_active: data.tab == self.this,
            is_hover: false,
        })
    }

    fn with_mut<V, F: FnOnce(&mut NavItemState) -> V>(&self, data: &mut AppState, f: F) -> V {
        f(&mut NavItemState {
            this: self.this,
            is_active: data.tab == self.this,
            is_hover: false,
        })
    }
}
