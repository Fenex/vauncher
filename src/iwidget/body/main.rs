use super::*;
use druid::{FileDialogOptions, FileInfo};

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

pub(super) const TAB_KEY: TabId = TabId::Main;

pub(super) fn template() -> impl Widget<AppState> {
    Container::new(item().lens(AppState::bases.index(0)))
        .padding(10.)
        .expand()
}

// pub fn template() -> impl Widget<AppState> {
//     Container::new(item().lens(AppState::bases.index(0)))
//         .padding(10.)
//         .expand()
// }

const BIN_PATH_CHANGED: Selector<FileInfo> = Selector::new("bin path has been changed");
const RSC_PATH_CHANGED: Selector<FileInfo> = Selector::new("resource path has been changed");

const PATHS_UPDATED: Selector<(PathBuf, PathBuf)> =
    Selector::new("Resource and\\or binary paths have been auto-updated");

fn dlg_choose_bin() -> druid::FileDialogOptions {
    FileDialogOptions::new()
        // .select_directories()
        .accept_command(BIN_PATH_CHANGED)
        .button_text("Choose binary")
}

fn dlg_choose_resource() -> druid::FileDialogOptions {
    FileDialogOptions::new()
        .select_directories()
        .accept_command(RSC_PATH_CHANGED)
        .button_text("Choose resources")
}

fn item() -> impl Widget<CardItemBase> {
    let name_lbl = Label::new("Имя: ")
        .align_right()
        .fix_width(200.)
        .padding(10.);
    let name_txt = TextBox::new()
        .with_placeholder("Card title")
        .lens(CardItemBase::name)
        .expand_width();

    let path_bin_lbl = Label::new("Исполняемый файл: ")
        .align_right()
        .fix_width(200.)
        .padding(10.);
    let path_bin_txt = TextBox::new()
        .with_placeholder("Choose an executable file -->")
        .lens(CardItemBase::path_bin)
        .expand_width();
    let path_bin_btn = Button::new("...").on_click(move |ctx, _, _env| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(dlg_choose_bin()));
    });

    let path_rsc_lbl = Label::new("Ресурсы: ")
        .align_right()
        .fix_width(200.)
        .padding(10.);
    let path_rsc_txt = TextBox::new()
        .with_placeholder("Choose resource folder -->")
        .lens(CardItemBase::path_resources)
        .expand_width();
    let path_bin_rsc = Button::new("...").on_click(move |ctx, _, _env| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(dlg_choose_resource()))
    });

    let name = Flex::row()
        .with_child(name_lbl)
        .with_flex_child(name_txt, 1.);
    let path_bin = Flex::row()
        .with_child(path_bin_lbl)
        .with_flex_child(path_bin_txt, 1.)
        .with_child(path_bin_btn.padding((10., 0., 0., 0.)));
    let path_rsc = Flex::row()
        .with_child(path_rsc_lbl)
        .with_flex_child(path_rsc_txt, 1.)
        .with_child(path_bin_rsc.padding((10., 0., 0., 0.)));

    Flex::column()
        .with_child(name.padding((10., 0.)))
        .with_child(path_bin.padding((10., 0.)))
        .with_child(path_rsc.padding((10., 0.)))
        .expand_width()
        .controller(ItemController)
}

struct ItemController;

impl<W: Widget<CardItemBase>> Controller<CardItemBase, W> for ItemController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx,
        event: &Event,
        data: &mut CardItemBase,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(BIN_PATH_CHANGED) => {
                data.path_bin = cmd
                    .get(BIN_PATH_CHANGED)
                    .unwrap()
                    .path()
                    .to_str()
                    .map(|s| s.to_owned())
                    .unwrap_or_default();
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(RSC_PATH_CHANGED) => {
                data.path_resources = cmd
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
                data.path_bin = bin_path.to_str().unwrap().to_string();
                data.path_resources = resource_path.to_str().unwrap().to_string();
                ctx.set_handled();
            }
            _ => (),
        }

        child.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &CardItemBase,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut druid::UpdateCtx,
        old_data: &CardItemBase,
        data: &CardItemBase,
        env: &Env,
    ) {
        if old_data.path_bin != data.path_bin {
            let path = Path::new(&data.path_bin);
            if let Some(paths) = do_correct_bin_path(path) {
                ctx.submit_command(PATHS_UPDATED.with(paths));
            }
        }
        child.update(ctx, old_data, data, env)
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
    dbg!(&input_path);

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
