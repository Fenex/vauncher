use druid::*;
use druid::theme::BACKGROUND_DARK;
use druid::widget::*;

mod iwidget;
use iwidget::*;

struct AppState;

pub fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder()
        // .debug_paint_layout()
    );

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(())
}

fn btn<T: Data>(text: &str) -> impl Widget<T> {
    Button::from_label(
        Label::new(text)
            .with_text_size(28.)
    )
    .fix_width(200.)
}

fn body<T: Data>() -> impl Widget<T> {
    Container::new(Label::new("body"))
        .expand()
}

fn bottom<T: Data>() -> impl Widget<T> {
    Container::new(
        Align::right(
            Flex::row()
                .with_child(btn("Play"))
                .with_spacer(15.)
                .with_child(btn("Exit"))
        ).padding(10.)
    )
}

fn nav<T: Data>() -> impl Widget<T> {
    Container::new(
            Flex::column()
                .with_child(Link::new("Mods").lens)
                .with_child(Label::new("Game settings"))
        )
        .fix_width(200.)
        .expand_height()
        .background(Color::rgb8(53, 53, 53))
}

fn ui_builder<T: Data>() -> impl Widget<T> {
    Flex::row()
        .with_child(nav())
        .with_flex_child(
            Flex::column()
                .with_flex_child(body(), 1.)
                .with_child(bottom()),
                1.0
        )
}