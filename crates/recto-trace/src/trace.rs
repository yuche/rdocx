use std::collections::BTreeMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::duration_serde;
use crate::feature_tag::FeatureTag;
use crate::format::FormatTag;
use crate::ids::{DocumentId, NodeId};
use crate::layout_map::LayoutMap;
use crate::loss::LossEvent;
use crate::node_map::NodeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FidelityTrace {
    pub document_id: DocumentId,
    pub source_format: FormatTag,
    pub target_format: FormatTag,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stages: Vec<StageTrace>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub loss_events: Vec<LossEvent>,

    #[serde(default)]
    pub node_map: NodeMap,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout_map: Option<LayoutMap>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StageTrace {
    pub stage: StageKind,

    #[serde(with = "duration_serde")]
    pub duration: Duration,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub metrics: BTreeMap<String, MetricValue>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub feature_tags: Vec<FeatureTag>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<TraceWarning>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum StageKind {
    Parse,
    Lower,
    Recover,
    Layout,
    Emit,
    Render,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum MetricValue {
    Count(u64),
    Ratio(f64),
    Bytes(u64),
    Duration(#[serde(with = "duration_serde")] Duration),
    Confidence(f32),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraceWarning {
    pub code: String,
    pub message: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<NodeId>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum TraceMode {
    #[default]
    Off,
    Summary,
    Full,
}

pub mod metric_keys {
    pub const PARSE_UNKNOWN_NODE_COUNT: &str = "unknown_node_count";
    pub const PARSE_DROPPED_NODE_COUNT: &str = "dropped_node_count";
    pub const PARSE_FALLBACK_DECODE_COUNT: &str = "fallback_decode_count";
    pub const PARSE_RECOVERY_USED: &str = "parse_recovery_used";

    pub const LOWER_CANONICAL_COVERAGE_RATIO: &str = "canonical_coverage_ratio";
    pub const LOWER_EXTENSION_PASSTHROUGH_RATIO: &str = "extension_passthrough_ratio";

    pub const LAYOUT_LINE_BREAK_COUNT: &str = "line_break_count";
    pub const LAYOUT_PAGE_COUNT: &str = "page_count";
    pub const LAYOUT_TABLE_PASS_COUNT: &str = "table_layout_pass_count";
    pub const LAYOUT_ANCHOR_COLLISION_COUNT: &str = "anchor_collision_count";
    pub const LAYOUT_FONT_SUBSTITUTION_COUNT: &str = "font_substitution_count";

    pub const EMIT_TAG_COUNT: &str = "tag_emit_count";
    pub const EMIT_SUBSET_GLYPH_COUNT: &str = "subset_glyph_count";
    pub const EMIT_TOUNICODE_COVERAGE: &str = "tounicode_coverage";

    pub const RENDER_PAGE_SSIM: &str = "page_ssim";
    pub const RENDER_REGION_DIFFS: &str = "region_diffs";
}
