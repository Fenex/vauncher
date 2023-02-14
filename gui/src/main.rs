use std::sync::Arc;

mod druid;
use ::vcore::Manager2;

use crate::druid::*;

mod vcore;
use crate::vcore::*;

mod iwidget;
use crate::iwidget::*;

mod l10n;
use crate::l10n::*;

mod prefs;
use crate::prefs::*;

pub fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());

    // let prefs = AppPrefs::init();
    // Manager2::n

    AppLauncher::with_window(main_window)
        .log_to_console()
        .localization_resources(vec!["lang.ftl".to_owned()], String::from("resources/i18n"))
        .configure_env(|env, _data| {
            let l10n = Localization::new(env);
            env.set(L10N, Arc::new(l10n));
        })
        .launch(AppState {
            prefs: AppPrefs::init(),
            tab: Default::default(),
            cards: Default::default(),
            mods: Default::default(),
        })
}
