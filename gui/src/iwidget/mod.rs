use crate::{druid::*, l10n::L10N};

mod body;
pub use body::*;

mod nav;
pub use nav::*;

mod _state;
pub use _state::*;

pub const HOT_CHANGED: Selector<bool> = Selector::new("widget hot changed (mouse on\\out events)");

pub(super) fn ui_builder() -> impl Widget<AppState> {
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
        .with_flex_child(body::template(), 1.)
}
