use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::ids::{FontHandle, NodeId};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LayoutMap {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub node_to_boxes: BTreeMap<NodeId, Vec<PageBBox>>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub glyph_to_source: Vec<GlyphSource>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub page_breaks: Vec<PageBreakDecision>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub table_decisions: Vec<TableLayoutDecision>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub anchor_decisions: Vec<AnchorPlacementDecision>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PageBBox {
    pub page_index: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlyphSource {
    pub glyph_index: u32,
    pub font_handle: FontHandle,
    pub source_node: NodeId,
    pub unicode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageBreakDecision {
    pub before_node: NodeId,
    pub reason: PageBreakReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum PageBreakReason {
    ExplicitBreak,
    PageBreakBefore,
    KeepLines,
    WidowControl,
    KeepNextChainStart,
    SectionBreak,
    PageOverflow,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableLayoutDecision {
    pub table_node: NodeId,
    pub layout_mode: TableLayoutMode,
    pub columns: Vec<TableColumnDecision>,
    pub passes_taken: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum TableLayoutMode {
    Autofit,
    Fixed,
    Preferred,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TableColumnDecision {
    pub min_content: f32,
    pub max_content: f32,
    pub final_width: f32,
    pub source: ColumnSource,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ColumnSource {
    Preferred,
    Calculated,
    Spread,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnchorPlacementDecision {
    pub anchor_node: NodeId,
    pub initial_position: PageBBox,
    pub final_position: PageBBox,
    pub displaced_pages: u32,
    pub collision_count: u32,
}
