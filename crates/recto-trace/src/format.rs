use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum FormatTag {
    Docx,
    Pdf,
    Html,
    Markdown,
    Odt,
    Rtf,
    Xlsx,
    Pptx,
    Unknown,
}
