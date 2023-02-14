use super::*;
use crate::{druid::*, iwidget::{Local, body::mods::CHANGE_TAB_MOD_ID, TabModsId, ADD_MOD}};

const LOCAL_PACKAGE_JSON_PATH_CHANGED: Selector<FileInfo> =
    Selector::new("local package.json path has been changed");
const LOCAL_PACKAGE_JSON_PATH_CHOOSE_CANCELED: Selector<()> =
    Selector::new("modal dialog to choose local package.json path was closed without any result");

use vcore::{ModificationConfig, ModificationSource, ModificationSourceLocal};

#[derive(Default, Debug, Clone, Lens)]
pub(super) struct LocalState {
    file: String,
    config: Option<ModificationConfig>,
}

impl Data for LocalState {
    fn same(&self, other: &Self) -> bool {
        use ModificationSource::Github as G;
        use ModificationSource::Local as L;

        let is_same_mod_config = match (&self.config, &other.config) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                let is_same_sources = match (&a.source, &b.source) {
                    (G(a), G(b)) => a == b,
                    (L(a), L(b)) => a == b,
                    _ => false,
                };

                let is_same_package = match (&a.package, &b.package) {
                    (None, None) => true,
                    (Some(a), Some(b)) => a == b,
                    _ => false,
                };

                is_same_sources && is_same_package
            }
            _ => false,
        };

        self.file == other.file && is_same_mod_config
    }
}

pub(super) fn template() -> impl Widget<LocalState> {
    Flex::column()
        .with_child(dl_row(
            "PATH_TO_PACKAGE_JSON:",
            Flex::row()
                .with_child(Button::new("Browse").on_click(|ctx, _, _| {
                    ctx.submit_command(commands::SHOW_OPEN_PANEL.with(dlg_choose_package_json()));
                }))
                .with_flex_child(
                    TextBox::new()
                        .with_placeholder("<- CHOSE_PACKAGE_JSON_FILE")
                        .disabled_if(|_, _| true)
                        .expand_width()
                        .lens(LocalState::file),
                    1.0,
                ),
        ))
        .with_child(Either::new(
            |x: &LocalState, _env| {
                x.config.is_some() && x.config.as_ref().unwrap().package.is_some()
            },
            dl_row(
                "PACKAGE_JSON_NAME:",
                Label::dynamic(|x: &LocalState, _env| {
                    x.config
                        .as_ref()
                        .and_then(|c| c.package.as_ref())
                        .map(|p| p.name.clone())
                        .unwrap_or(String::from("UNKNOWN"))
                }),
            ),
            Either::new(|x: &LocalState, _env| x.file.is_empty(),
                Label::new(""),
                Label::new("WRONG_PACKAGE_JSON")
            )
        ))
        .with_child(
            Button::new("Add")
            .disabled_if(|x: &LocalState, _env| x.config.as_ref().and_then(|c| c.package.as_ref()).is_none())
            .on_click(|ctx: &mut EventCtx, x, _env| {
                dbg!(&x);
                if let Some(config) = x.config.take() {
                    ctx.submit_command(ADD_MOD.with(config));
                }
            })
        )
        .controller(AddModLocalController)
}

fn dlg_choose_package_json() -> FileDialogOptions {
    FileDialogOptions::new()
        .accept_command(LOCAL_PACKAGE_JSON_PATH_CHANGED)
        .cancel_command(LOCAL_PACKAGE_JSON_PATH_CHOOSE_CANCELED)
        .button_text("Choose binary")
}

struct AddModLocalController;

impl<W: Widget<LocalState>> Controller<LocalState, W> for AddModLocalController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut LocalState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(LOCAL_PACKAGE_JSON_PATH_CHANGED) => {
                let file = cmd.get_unchecked(LOCAL_PACKAGE_JSON_PATH_CHANGED);
                data.file = file.path().to_str().unwrap().into();
                let source = ModificationSourceLocal(data.file.clone());
                let config = ModificationConfig::new(ModificationSource::Local(source));
                data.config = config;
            },
            _ => (),
        }
        child.event(ctx, event, data, env)
    }
}
