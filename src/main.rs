use std::sync::Arc;

use druid::im::vector;
use druid::widget::*;
use druid::*;

mod iwidget;
use iwidget::*;

mod nav;
use nav::*;

mod l10n;
use l10n::*;

#[derive(Debug, Default, Clone, Data, Lens)]
pub struct AppState {
    pub tab: TabId,
    pub cards: im::Vector<iwidget::CardItem>,
}

pub fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());

    AppLauncher::with_window(main_window)
        .log_to_console()
        .localization_resources(vec!["lang.ftl".to_owned()], String::from("resources/i18n"))
        .configure_env(|env, _data| {
            let l10n = Localization::new(env);
            env.set(L10N, Arc::new(l10n));
        })
        .launch(AppState {
            cards: vector![],
            ..Default::default()
        })
}

fn ui_builder() -> impl Widget<AppState> {
    Flex::row()
        .with_child(
            Flex::column()
                .with_flex_child(nav(), 1.)
                .with_child(
                    Button::from_label(
                        Label::dynamic(|_data, env| env.get(L10N).get("footer-button-exit"))
                            .with_text_size(28.),
                    )
                    .on_click(|_, _, _| Application::global().quit())
                    .padding(5.0),
                )
                .cross_axis_alignment(CrossAxisAlignment::Fill)
                .background(Color::rgb8(53, 53, 53)),
        )
        .with_flex_child(body(), 1.)
}
