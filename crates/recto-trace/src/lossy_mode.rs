use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum LossyMode {
    Strict,
    #[default]
    WarnLog,
    AllowWarnings,
    Silent,
}
