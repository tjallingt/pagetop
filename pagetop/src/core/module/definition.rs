use crate::core::action::Action;
use crate::core::theme::ThemeRef;
use crate::locale::L10n;
use crate::{actions, service, util, HasHandle};

#[cfg(feature = "database")]
use crate::{db::MigrationItem, migrations};

pub type ModuleRef = &'static dyn ModuleTrait;

pub trait ModuleBase {
    fn single_name(&self) -> &'static str;
}

/// Los módulos deben implementar este *trait*.
pub trait ModuleTrait: HasHandle + ModuleBase + Send + Sync {
    fn name(&self) -> L10n {
        L10n::n(self.single_name())
    }

    fn description(&self) -> L10n {
        L10n::default()
    }

    fn theme(&self) -> Option<ThemeRef> {
        None
    }

    fn dependencies(&self) -> Vec<ModuleRef> {
        vec![]
    }

    fn drop_modules(&self) -> Vec<ModuleRef> {
        vec![]
    }

    fn actions(&self) -> Vec<Action> {
        actions![]
    }

    fn init(&self) {}

    #[cfg(feature = "database")]
    #[allow(unused_variables)]
    fn migrations(&self) -> Vec<MigrationItem> {
        migrations![]
    }

    #[allow(unused_variables)]
    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {}
}

impl<M: ?Sized + ModuleTrait> ModuleBase for M {
    fn single_name(&self) -> &'static str {
        util::single_type_name::<Self>()
    }
}
