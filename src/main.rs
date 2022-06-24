use std::sync::Arc;

use druid::*;
use druid::widget::*;
use druid::im::vector;

mod iwidget;
use iwidget::*;

mod nav;
use nav::*;

mod l10n;
use l10n::*;


#[derive(Debug, Default, Clone, Data, Lens)]
pub struct AppState {
    tab: TabId,
    bases: im::Vector<iwidget::CardItemBase>,
}

pub fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(
        ui_builder(), // .debug_paint_layout()
    );

    AppLauncher::with_window(main_window)
        .log_to_console()
        .localization_resources(vec!["lang.ftl".to_owned()], String::from("resources/i18n"))
        .configure_env(|env, _data| {
            let l10n = Localization::new(env);
            env.set(L10N, Arc::new(l10n));
        })
        .launch(AppState {
            bases: vector![CardItemBase::default()],
            ..Default::default()
        })
}

fn ui_builder() -> impl Widget<AppState> {
    Flex::row().with_child(nav()).with_flex_child(
        Flex::column()
            .with_flex_child(body(), 1.)
            .with_child(bottom()),
        1.0,
    )
}

fn btn<T: Data>(text_key: &'static str) -> impl Widget<T> {
    Button::from_label(Label::dynamic(|_data, env| env.get(L10N).get(text_key)).with_text_size(28.))
        .fix_width(200.)
}

fn bottom<T: Data>() -> impl Widget<T> {
    Container::new(
        Align::right(
            Flex::row()
                .with_child(btn("footer-button-play"))
                .with_spacer(15.)
                .with_child(btn("footer-button-exit")),
        )
        .padding(10.),
    )
}
