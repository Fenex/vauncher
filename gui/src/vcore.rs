use std::sync::Arc;
use ::druid::Data;

pub use ::vcore::ModificationConfig;

#[derive(Debug, Clone, PartialEq)]
pub struct ModificationConfigD(pub ModificationConfig);

impl Data for ModificationConfigD {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl From<ModificationConfig> for ModificationConfigD {
    fn from(v: ModificationConfig) -> Self {
        Self(v)
    }
}

impl From<ModificationConfigD> for ModificationConfig {
    fn from(v: ModificationConfigD) -> Self {
        v.0
    }
}

impl AsRef<ModificationConfig> for ModificationConfigD {
    fn as_ref(&self) -> &ModificationConfig {
        &self.0
    }
}

