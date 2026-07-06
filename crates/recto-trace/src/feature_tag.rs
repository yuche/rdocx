use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FeatureTag(pub String);

impl FeatureTag {
    pub fn new(tag: impl Into<String>) -> Self {
        Self(tag.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether this tag is in the registered vocabulary ([`known::ALL`]).
    ///
    /// The regression corpus's `validate-corpus` rejects metadata whose
    /// `features` entries are not known; adding a tag means editing
    /// [`known`] and going through a main-repo PR.
    pub fn is_known(&self) -> bool {
        known::ALL.contains(&self.0.as_str())
    }
}

impl From<&str> for FeatureTag {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

/// Registered feature-tag vocabulary (authoritative; docs point here).
///
/// Corpus metadata (`features:`) and `FidelityTrace.stages[].feature_tags`
/// must draw from this set so per-feature SSIM aggregation stays meaningful.
pub mod known {
    // Tables
    pub const TABLES: &str = "features/tables";
    pub const TABLES_AUTOFIT: &str = "features/tables/autofit";
    pub const TABLES_MERGED: &str = "features/tables/merged";
    pub const TABLES_NESTED: &str = "features/tables/nested";
    pub const TABLES_REPEAT_HEADER: &str = "features/tables/repeat-header";

    // Anchored objects
    pub const ANCHORS: &str = "features/anchors";
    pub const ANCHORS_WRAP_SQUARE: &str = "features/anchors/wrap-square";
    pub const ANCHORS_WRAP_TIGHT: &str = "features/anchors/wrap-tight";
    pub const ANCHORS_WRAP_THROUGH: &str = "features/anchors/wrap-through";
    pub const ANCHORS_WRAP_TOP_BOTTOM: &str = "features/anchors/wrap-top-bottom";

    // East Asian typography
    pub const CJK: &str = "features/cjk";
    pub const CJK_KINSOKU: &str = "features/cjk/kinsoku";
    pub const CJK_VERTICAL: &str = "features/cjk/vertical";
    pub const CJK_AUTO_SPACE_DE: &str = "features/cjk/auto-space-de";
    pub const CJK_AUTO_SPACE_DN: &str = "features/cjk/auto-space-dn";

    // Math
    pub const MATH_OMML: &str = "features/math/omml";
    pub const MATH_MATHML: &str = "features/math/mathml";
    pub const MATH_LATEX: &str = "features/math/latex";

    // Document structure
    pub const FOOTNOTES: &str = "features/footnotes";
    pub const ENDNOTES: &str = "features/endnotes";
    pub const COMMENTS: &str = "features/comments";
    pub const REVISIONS: &str = "features/revisions";
    pub const SDT: &str = "features/sdt";
    pub const FIELDS: &str = "features/fields";
    pub const FIELDS_MERGEFIELD: &str = "features/fields/mergefield";
    pub const FIELDS_PAGEREF: &str = "features/fields/pageref";
    pub const FIELDS_TOC: &str = "features/fields/toc";
    pub const SECTIONS: &str = "features/sections";
    pub const COLUMNS: &str = "features/columns";

    // PDF read side (post-gate; registered so corpus can pre-tag)
    pub const PDF_TAGGED: &str = "features/pdf/tagged";
    pub const PDF_UNTAGGED: &str = "features/pdf/untagged";
    pub const PDF_SCANNED: &str = "features/pdf/scanned";
    pub const PDF_MULTI_COLUMN: &str = "features/pdf/multi-column";

    // Markdown / HTML (post-gate)
    pub const MARKDOWN_GFM: &str = "features/markdown/gfm";
    pub const MARKDOWN_RAW_HTML: &str = "features/markdown/raw-html";
    pub const HTML_CSS_INLINE: &str = "features/html/css-inline";

    pub const ALL: &[&str] = &[
        TABLES,
        TABLES_AUTOFIT,
        TABLES_MERGED,
        TABLES_NESTED,
        TABLES_REPEAT_HEADER,
        ANCHORS,
        ANCHORS_WRAP_SQUARE,
        ANCHORS_WRAP_TIGHT,
        ANCHORS_WRAP_THROUGH,
        ANCHORS_WRAP_TOP_BOTTOM,
        CJK,
        CJK_KINSOKU,
        CJK_VERTICAL,
        CJK_AUTO_SPACE_DE,
        CJK_AUTO_SPACE_DN,
        MATH_OMML,
        MATH_MATHML,
        MATH_LATEX,
        FOOTNOTES,
        ENDNOTES,
        COMMENTS,
        REVISIONS,
        SDT,
        FIELDS,
        FIELDS_MERGEFIELD,
        FIELDS_PAGEREF,
        FIELDS_TOC,
        SECTIONS,
        COLUMNS,
        PDF_TAGGED,
        PDF_UNTAGGED,
        PDF_SCANNED,
        PDF_MULTI_COLUMN,
        MARKDOWN_GFM,
        MARKDOWN_RAW_HTML,
        HTML_CSS_INLINE,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_vocabulary_is_recognized() {
        assert!(FeatureTag::from(known::TABLES_AUTOFIT).is_known());
        assert!(FeatureTag::from(known::CJK_KINSOKU).is_known());
        assert!(!FeatureTag::from("features/markdown/myst").is_known());
    }

    #[test]
    fn vocabulary_has_no_duplicates_and_consistent_prefix() {
        let mut seen = std::collections::HashSet::new();
        for tag in known::ALL {
            assert!(tag.starts_with("features/"), "bad prefix: {tag}");
            assert!(seen.insert(*tag), "duplicate tag: {tag}");
        }
    }
}
