use crate::iwidget::HOT_CHANGED;

use super::*;
use druid::{
    lens::Constant, text::RichTextBuilder, FileDialogOptions, FileInfo, Insets, LifeCycle, Target,
};

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

pub(super) const TAB_KEY: TabId = TabId::Main;

pub(super) fn template() -> impl Widget<AppState> {
    Either::new(
        |data: &AppState, _| data.cards.is_empty(),
        {
            let mut builder = RichTextBuilder::new();
            builder
                .push("Добавьте")
                .underline(true)
                .text_color(Color::rgb8(50, 50, 160))
                .link(ADD_NEW_CARD);
            builder.push(" карточку запуска");

            let text = RawLabel::new()
                .with_text_size(40.)
                .with_text_color(Color::GRAY)
                .with_line_break_mode(LineBreaking::WordWrap)
                .lens(Constant(builder.build()))
                .controller(EmptyCardsController);

            Flex::column()
                .with_child(text)
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .main_axis_alignment(MainAxisAlignment::Center)
                .expand()
        },
        body(),
    )
}

fn body() -> impl Widget<AppState> {
    Flex::column()
        .with_flex_child(
            Scroll::new(
                List::new(|| {
                    item()
                        .padding(10.)
                        .border(Color::GRAY, 1.)
                        .rounded(3.)
                        .padding(Insets::new(0., 0., 0., 5.))
                })
                .lens(AppState::cards),
            )
            .vertical()
            .expand_height()
            .controller(ListCardItemsController),
            1.0,
        )
        .with_child(
            Button::new("New").on_click(move |_ctx, data: &mut AppState, _env| {
                data.cards.push_back(CardItem::new().with_edit());
            }),
        )
}

const ADD_NEW_CARD: Selector = Selector::new("show a template to create new card");

struct EmptyCardsController;

impl<W: Widget<AppState>> Controller<AppState, W> for EmptyCardsController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(ADD_NEW_CARD) => {
                data.cards.push_back(CardItem::new().with_edit());
                ctx.set_handled();
            }
            _ => (),
        }
        child.event(ctx, event, data, env)
    }
}

const BIN_PATH_CHANGED: Selector<FileInfo> = Selector::new("bin path has been changed");
const RSC_PATH_CHANGED: Selector<FileInfo> = Selector::new("resource path has been changed");
const BIN_PATH_CHOOSE_STARTED: Selector<u32> =
    Selector::new("modal dialog to choose bin path was opened");
const RSC_PATH_CHOOSE_STARTED: Selector<u32> =
    Selector::new("modal dialog to choose resource path was opened");
const BIN_PATH_CHOOSE_CANCELED: Selector<()> =
    Selector::new("modal dialog to choose bin path was closed without any result");
const RSC_PATH_CHOOSE_CANCELED: Selector<()> =
    Selector::new("modal dialog to choose resource path was closed without any result");
const REMOVE_CARD_ITEM: Selector<u32> = Selector::new("request for remove card item with that id");
const PATHS_UPDATED: Selector<(PathBuf, PathBuf)> =
    Selector::new("Resource and\\or binary paths have been auto-updated");

fn dlg_choose_bin() -> druid::FileDialogOptions {
    FileDialogOptions::new()
        // .select_directories()
        .accept_command(BIN_PATH_CHANGED)
        .cancel_command(BIN_PATH_CHOOSE_CANCELED)
        .button_text("Choose binary")
}

fn dlg_choose_resource() -> druid::FileDialogOptions {
    FileDialogOptions::new()
        .select_directories()
        .accept_command(RSC_PATH_CHANGED)
        .cancel_command(RSC_PATH_CHOOSE_CANCELED)
        .button_text("Choose resources")
}

fn item() -> impl Widget<CardItem> {
    ViewSwitcher::new(
        |data: &CardItem, _| data.edit.is_some(),
        |&selector, _, _| match selector {
            true => item_edit().boxed(),
            false => item_read().boxed(),
        },
    )
}

fn item_edit() -> impl Widget<CardItem> {
    let name_lbl = Label::new("Имя: ").padding(10.);
    let name_txt = TextBox::new()
        .with_placeholder("Card title")
        .lens(CardItemEdit.then(CardItemRecord::name))
        .expand_width();

    let path_bin_lbl = Label::new("Исполняемый файл: ").padding(10.);
    let path_bin_txt = TextBox::new()
        .with_placeholder("Choose an executable file -->")
        .lens(CardItemEdit.then(CardItemRecord::path_bin))
        .expand_width();

    let path_bin_btn = Button::new("...").on_click(move |ctx, data: &mut CardItem, _env| {
        ctx.submit_command(BIN_PATH_CHOOSE_STARTED.with(data.id));
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(dlg_choose_bin()));
    });

    let path_rsc_lbl = Label::new("Ресурсы: ").padding(10.);
    let path_rsc_txt = TextBox::new()
        .with_placeholder("Choose resource folder -->")
        .lens(CardItemEdit.then(CardItemRecord::path_resources))
        .expand_width();
    let path_rsc_btn = Button::new("...").on_click(move |ctx, data: &mut CardItem, _env| {
        ctx.submit_command(RSC_PATH_CHOOSE_STARTED.with(data.id));
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(dlg_choose_resource()));
    });

    const ROW_HEIGHT: f64 = 40.;

    let btn_cancel = Button::new("Cancel").on_click(|_ctx, data: &mut CardItem, _env| {
        data.edit = None;
    });

    let btn_save = Button::new("Save").on_click(|_ctx, data: &mut CardItem, _env| {
        data.record = data.edit.take().unwrap();
    });

    let btn_controls = Flex::row()
        .with_child(btn_cancel)
        .with_spacer(5.)
        .with_child(btn_save)
        .expand_width();

    let body = Flex::row()
        .with_child(
            Flex::column()
                .with_child(name_lbl.fix_height(ROW_HEIGHT))
                .with_child(path_bin_lbl.fix_height(ROW_HEIGHT))
                .with_child(path_rsc_lbl.fix_height(ROW_HEIGHT))
                .cross_axis_alignment(CrossAxisAlignment::End),
        )
        .with_flex_child(
            Flex::column()
                .with_child(
                    Flex::row()
                        .with_flex_child(name_txt, 1.0)
                        .fix_height(ROW_HEIGHT),
                )
                .with_child(
                    Flex::row()
                        .with_flex_child(path_bin_txt, 1.0)
                        .with_spacer(10.)
                        .with_child(path_bin_btn)
                        .fix_height(ROW_HEIGHT),
                )
                .with_child(
                    Flex::row()
                        .with_flex_child(path_rsc_txt, 1.0)
                        .with_spacer(10.)
                        .with_child(path_rsc_btn)
                        .fix_height(ROW_HEIGHT),
                ),
            1.,
        );

    Flex::column()
        .with_child(body)
        .with_child(btn_controls)
        .cross_axis_alignment(CrossAxisAlignment::End)
        .controller(CardItemController::default())
}

struct CardItemReadTitle;
impl Lens<CardItemRecord, String> for CardItemReadTitle {
    fn with<V, F: FnOnce(&String) -> V>(&self, data: &CardItemRecord, f: F) -> V {
        let mut out = String::from(&data.name);
        if !data.path_bin.is_empty() {
            out += " (";
            out += &data.path_bin;
            out += ")";
        }
        f(&out)
    }

    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, data: &mut CardItemRecord, f: F) -> V {
        let mut out = String::from(&data.name);
        if !data.path_bin.is_empty() {
            out += " (";
            out += &data.path_bin;
            out += ")";
        }
        f(&mut out)
    }
}

fn item_read() -> impl Widget<CardItem> {
    let name_txt = TextBox::new()
        .with_placeholder("[empty]")
        .disabled_if(|_, _| true)
        .lens(CardItemRead.then(CardItemReadTitle))
        .expand_width();

    const ROW_HEIGHT: f64 = 40.;

    let btn_edit = Button::new("Edit").on_click(|_ctx, data: &mut CardItem, _env| {
        data.edit = Some(data.record.clone());
    });

    let btn_delete = Button::new("Delete").on_click(|ctx, data: &mut CardItem, _env| {
        ctx.submit_notification(REMOVE_CARD_ITEM.with(data.id));
    });

    let btn_controls = Either::new(
        |data: &CardItem, _env| data.is_hover,
        Flex::row()
            .with_child(btn_edit)
            .with_spacer(5.)
            .with_child(btn_delete),
        Flex::row().with_spacer(1.),
    )
    .fix_height(ROW_HEIGHT);

    Flex::row()
        .with_child(
            Flex::column()
                .with_child(Button::new("P").fix_size(ROW_HEIGHT * 2. - 8., ROW_HEIGHT * 2. - 8.)),
        )
        .with_spacer(10.)
        .with_flex_child(
            Flex::column()
                .with_child(
                    Flex::row()
                        .with_flex_child(name_txt, 1.0)
                        .fix_height(ROW_HEIGHT),
                )
                .with_child(btn_controls)
                .cross_axis_alignment(CrossAxisAlignment::End),
            1.,
        )
        .controller(CardItemController::default())
    // .debug_paint_layout()
}

struct ListCardItemsController;

impl<W: Widget<AppState>> Controller<AppState, W> for ListCardItemsController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Notification(cmd) if cmd.is(REMOVE_CARD_ITEM) => {
                let id = *cmd.get(REMOVE_CARD_ITEM).unwrap();
                data.cards.retain(|item| item.id != id);
            }
            _ => (),
        }
        child.event(ctx, event, data, env)
    }
}

#[derive(Debug, Default)]
struct CardItemController {
    is_dialog_opened_for_this_item: bool,
}

impl<W: Widget<CardItem>> Controller<CardItem, W> for CardItemController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx,
        event: &Event,
        data: &mut CardItem,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(BIN_PATH_CHOOSE_STARTED) => {
                self.is_dialog_opened_for_this_item =
                    cmd.get(BIN_PATH_CHOOSE_STARTED).unwrap() == &data.id;
                println!(
                    "is_me: {}, {}",
                    &data.id, self.is_dialog_opened_for_this_item
                );
            }
            Event::Command(cmd) if cmd.is(RSC_PATH_CHOOSE_STARTED) => {
                self.is_dialog_opened_for_this_item =
                    cmd.get(RSC_PATH_CHOOSE_STARTED).unwrap() == &data.id;
            }
            Event::Command(cmd)
                if cmd.is(BIN_PATH_CHOOSE_CANCELED) || cmd.is(RSC_PATH_CHOOSE_CANCELED) =>
            {
                self.is_dialog_opened_for_this_item = false;
            }
            Event::Command(cmd)
                if cmd.is(BIN_PATH_CHANGED) && self.is_dialog_opened_for_this_item =>
            {
                self.is_dialog_opened_for_this_item = false;
                data.edit.as_mut().unwrap().path_bin = cmd
                    .get(BIN_PATH_CHANGED)
                    .unwrap()
                    .path()
                    .to_str()
                    .map(|s| s.to_owned())
                    .unwrap_or_default();
                ctx.set_handled();
            }
            Event::Command(cmd)
                if cmd.is(RSC_PATH_CHANGED) && self.is_dialog_opened_for_this_item =>
            {
                self.is_dialog_opened_for_this_item = false;
                data.edit.as_mut().unwrap().path_resources = cmd
                    .get(RSC_PATH_CHANGED)
                    .unwrap()
                    .path()
                    .to_str()
                    .map(|s| s.to_owned())
                    .unwrap_or_default();
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(PATHS_UPDATED) => {
                let (bin_path, resource_path) = cmd.get(PATHS_UPDATED).unwrap();
                data.edit.as_mut().unwrap().path_bin = bin_path.to_str().unwrap().to_string();
                data.edit.as_mut().unwrap().path_resources =
                    resource_path.to_str().unwrap().to_string();
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(HOT_CHANGED) => {
                data.is_hover = *cmd.get(HOT_CHANGED).unwrap();
                ctx.set_handled();
            }
            _ => (),
        }

        child.event(ctx, event, data, env)
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut druid::UpdateCtx,
        old_data: &CardItem,
        data: &CardItem,
        env: &Env,
    ) {
        if let Some(ref edit) = data.edit {
            if old_data.edit.is_none() || old_data.edit.as_ref().unwrap().path_bin != edit.path_bin
            {
                let path = Path::new(&edit.path_bin);
                if let Some(paths) = do_correct_bin_path(path) {
                    let cmd = Command::new(PATHS_UPDATED, paths, Target::Widget(ctx.widget_id()));
                    ctx.submit_command(cmd);
                }
            }
        }
        child.update(ctx, old_data, data, env)
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut druid::LifeCycleCtx,
        event: &LifeCycle,
        data: &CardItem,
        env: &Env,
    ) {
        match event {
            LifeCycle::HotChanged(e) => {
                ctx.submit_command(HOT_CHANGED.with(*e).to(Target::Widget(ctx.widget_id())));
            }
            _ => (),
        }
        child.lifecycle(ctx, event, data, env)
    }
}

/// Returns corrected path:
/// 0: bin path
/// 1: resource path
fn do_correct_bin_path<P: AsRef<Path>>(path: P) -> Option<(PathBuf, PathBuf)> {
    const CHECK_RESOURCE_BY_FILE: &str = "common.prm";
    const BIN_VANGERS: &str = "vangers";
    const BIN_SURMAP: &str = "surmap";

    fn get_path_to_binary_in_current_dir(dir: &Path) -> Option<PathBuf> {
        if !dir.is_dir() {
            return None;
        }

        let mut out = None;

        if let Ok(read_dir) = dir.read_dir() {
            for path in read_dir
                .filter_map(|f| f.ok())
                .filter(|f| f.path().is_file())
                .map(|f| f.path())
                .inspect(|x| {
                    dbg!(&x);
                })
                .filter(|p| p.file_starts_with(BIN_VANGERS) || p.file_starts_with(BIN_SURMAP))
            {
                if out.is_none() {
                    out = Some(path);
                } else if path.file_starts_with(BIN_SURMAP) && dir.file_starts_with(BIN_SURMAP)
                    || path.file_starts_with(BIN_VANGERS) && dir.file_starts_with(BIN_VANGERS)
                {
                    match (dir.file_name(), path.file_name()) {
                        (Some(old), Some(new)) if old.len() > new.len() => {
                            out = Some(path);
                        }
                        _ => (),
                    }
                } else if path.file_starts_with(BIN_VANGERS)
                /* && !dir.starts_with(BIN_VANGERS)*/
                {
                    out = Some(path);
                }
            }
        }

        out
    }

    let mut input_path = Cow::Borrowed(path.as_ref());
    if input_path.is_dir() {
        let mut dirup = PathBuf::from(&*input_path);
        'external: while let Some(dir) = dirup.parent() {
            let checks = vec![
                dir.into(),
                dir.join("bin"),
                dir.join("bin").join("windows-64"),
                dir.join("bin").join("windows-32"),
            ];

            for bin_dir in checks {
                if let Some(bin_path) = get_path_to_binary_in_current_dir(&bin_dir) {
                    input_path = Cow::Owned(bin_path);
                    break 'external;
                }
            }

            dirup.pop();
        }
    }

    if input_path.is_file() {
        let mut dirup = PathBuf::from(&*input_path);
        while let Some(dir) = dirup.parent() {
            let checks = vec![
                dir.join(CHECK_RESOURCE_BY_FILE),
                dir.join("data").join(CHECK_RESOURCE_BY_FILE),
                dir.join("Data").join(CHECK_RESOURCE_BY_FILE),
                dir.join("DATA").join(CHECK_RESOURCE_BY_FILE),
            ];

            let expected_resource_dir = checks
                .iter()
                .find(|&p| p.is_file())
                .and_then(|p| p.parent())
                .map(|p| p.to_owned());

            if let Some(resource_dir) = expected_resource_dir {
                return Some((input_path.to_path_buf(), resource_dir));
            }

            dirup.pop();
        }
    }

    None
}

trait FileStartsWith {
    fn file_starts_with<T: AsRef<str>>(&self, str: T) -> bool;
}

impl<P: AsRef<Path>> FileStartsWith for P {
    fn file_starts_with<T: AsRef<str>>(&self, str: T) -> bool {
        self.as_ref()
            .to_str()
            .map(|s| s.starts_with(str.as_ref()))
            .unwrap_or(false)
    }
}
