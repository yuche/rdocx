//! Conversion fidelity trace, loss events, and regression schema.
//!
//! This crate is the stable JSON schema that the regression harness
//! (`specs/REGRESSION.md`) consumes. It defines:
//! - [`LossEvent`] / [`LossCode`] / [`LossyMode`] — explicit lossy negotiation.
//! - [`FidelityTrace`] / [`StageTrace`] / [`StageKind`] / [`MetricValue`] — observability.
//! - [`NodeMap`] / [`LayoutMap`] — source ↔ output correspondence for region-level diff.
//! - [`TraceMode`] — runtime control of trace verbosity.
//!
//! `recto::Document::convert_with_trace` (planned, `specs/GATE.md` FR-API-04 — not
//! implemented yet) will return a [`FidelityTrace`]; the regression harness
//! deserializes the JSON via this crate and aggregates dashboards.
//!
//! Stability contract: the JSON schema is NOT frozen until the regression repo's
//! `recto-diff` actually consumes it via git path (`specs/GATE.md` MS-A7). After
//! that, breaking changes require a coordinated PR in both this repo and the
//! regression repo (`specs/GATE.md` FR-FID-07).

mod duration_serde;
mod feature_tag;
mod format;
mod ids;
mod layout_map;
mod loss;
mod lossy_mode;
mod node_map;
mod trace;

pub use feature_tag::FeatureTag;
pub use format::FormatTag;
pub use ids::{DocumentId, FontHandle, MarkedContentId, NodeId, OriginPath};
pub use layout_map::{
    AnchorPlacementDecision, ColumnSource, GlyphSource, LayoutMap, PageBBox, PageBreakDecision,
    PageBreakReason, TableColumnDecision, TableLayoutDecision, TableLayoutMode,
};
pub use loss::{LossCode, LossEvent, LossSeverity, LossSuggestion};
pub use lossy_mode::LossyMode;
pub use node_map::NodeMap;
pub use trace::{
    metric_keys, FidelityTrace, MetricValue, StageKind, StageTrace, TraceMode, TraceWarning,
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn loss_event_json_round_trip() {
        let event = LossEvent {
            code: LossCode::FontMetricMismatch {
                source: "Calibri".into(),
                used: "Carlito".into(),
                similarity: 0.92,
            },
            severity: LossSeverity::Warning,
            source_node: Some(NodeId(42)),
            source_format: FormatTag::Docx,
            target_format: FormatTag::Pdf,
            message: "metric-compatible substitution".into(),
            suggestions: vec![LossSuggestion::AddFont {
                family: "Calibri".into(),
            }],
        };

        let json = serde_json::to_string(&event).unwrap();
        let decoded: LossEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.code, event.code);
        assert_eq!(decoded.severity, event.severity);
        assert_eq!(decoded.source_node, event.source_node);
    }

    #[test]
    fn fidelity_trace_minimum() {
        let trace = FidelityTrace {
            document_id: DocumentId(1),
            source_format: FormatTag::Docx,
            target_format: FormatTag::Pdf,
            stages: vec![StageTrace {
                stage: StageKind::Parse,
                duration: Duration::from_millis(12),
                metrics: Default::default(),
                feature_tags: vec![FeatureTag::from("features/tables/autofit")],
                warnings: vec![],
            }],
            loss_events: vec![],
            node_map: NodeMap::default(),
            layout_map: None,
        };

        let json = serde_json::to_string(&trace).unwrap();
        let decoded: FidelityTrace = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.stages.len(), 1);
        assert_eq!(decoded.stages[0].duration, Duration::from_millis(12));
    }

    #[test]
    fn loss_code_stable_ids() {
        assert_eq!(
            LossCode::UnknownExtensionDropped.stable_id(),
            "RECTO-LOSS-1001"
        );
        assert_eq!(LossCode::VbaMacroDropped.stable_id(), "RECTO-LOSS-2003");
        assert_eq!(LossCode::OpcUnknownPart.stable_id(), "RECTO-LOSS-7001");
    }
}
