use serde::{Deserialize, Serialize};

use crate::format::FormatTag;
use crate::ids::NodeId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LossEvent {
    pub code: LossCode,
    pub severity: LossSeverity,
    pub source_node: Option<NodeId>,
    pub source_format: FormatTag,
    pub target_format: FormatTag,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub suggestions: Vec<LossSuggestion>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LossSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LossSuggestion {
    EnableLossyMode { mode: crate::lossy_mode::LossyMode },
    UseConvertOption { field: String, value: String },
    AddFont { family: String },
    DowngradePdfProfile { from: String, to: String },
    UpgradeInputFormat { hint: String },
    Other { detail: String },
}

/// Stable lossy event identifier.
///
/// Numbering scheme (`RECTO-LOSS-NNNN`):
/// - `1xxx` Round-trip
/// - `2xxx` DOCX → PDF
/// - `3xxx` DOCX → HTML (added by spec 007)
/// - `4xxx` DOCX → Markdown (added by spec 006)
/// - `5xxx` PDF read (added by spec 005)
/// - `6xxx` Security / encryption
/// - `7xxx` OPC / relationships
/// - `9xxx` Generic
///
/// `#[non_exhaustive]` — downstream specs (005/006/007) add their own variants
/// without breaking SemVer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "code", rename_all = "PascalCase")]
pub enum LossCode {
    UnknownExtensionDropped,
    NamespacePrefixChanged,
    AttrOrderChanged,

    OleObjectNoPreview,
    SmartArtNoPreview,
    VbaMacroDropped,
    ActiveXDropped,
    FontMetricMismatch {
        source: String,
        used: String,
        similarity: f32,
    },
    UnknownFieldNotResolved,
    UpdateableFieldNotUpdated,
    Word97QuirkApproximated,

    OpcUnknownPart,
    OpcDanglingReference,

    Other {
        detail: String,
    },
}

impl LossCode {
    pub fn stable_id(&self) -> &'static str {
        match self {
            LossCode::UnknownExtensionDropped => "RECTO-LOSS-1001",
            LossCode::NamespacePrefixChanged => "RECTO-LOSS-1002",
            LossCode::AttrOrderChanged => "RECTO-LOSS-1003",
            LossCode::OleObjectNoPreview => "RECTO-LOSS-2001",
            LossCode::SmartArtNoPreview => "RECTO-LOSS-2002",
            LossCode::VbaMacroDropped => "RECTO-LOSS-2003",
            LossCode::ActiveXDropped => "RECTO-LOSS-2004",
            LossCode::FontMetricMismatch { .. } => "RECTO-LOSS-2005",
            LossCode::UnknownFieldNotResolved => "RECTO-LOSS-2006",
            LossCode::UpdateableFieldNotUpdated => "RECTO-LOSS-2007",
            LossCode::Word97QuirkApproximated => "RECTO-LOSS-2030",
            LossCode::OpcUnknownPart => "RECTO-LOSS-7001",
            LossCode::OpcDanglingReference => "RECTO-LOSS-7002",
            LossCode::Other { .. } => "RECTO-LOSS-9999",
        }
    }
}
