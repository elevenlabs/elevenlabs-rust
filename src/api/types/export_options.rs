pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "format")]
#[non_exhaustive]
pub enum ExportOptions {
    #[serde(rename = "docx")]
    Docx {
        #[serde(flatten)]
        data: DocxExportOptions,
    },

    #[serde(rename = "html")]
    Html {
        #[serde(flatten)]
        data: HtmlExportOptions,
    },

    #[serde(rename = "pdf")]
    Pdf {
        #[serde(flatten)]
        data: PdfExportOptions,
    },

    #[serde(rename = "segmented_json")]
    SegmentedJson {
        #[serde(flatten)]
        data: SegmentedJsonExportOptions,
    },

    #[serde(rename = "srt")]
    Srt {
        #[serde(flatten)]
        data: SrtExportOptions,
    },

    #[serde(rename = "txt")]
    Txt {
        #[serde(flatten)]
        data: TxtExportOptions,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ExportOptions {
    pub fn is_docx(&self) -> bool {
        matches!(self, Self::Docx { .. })
    }

    pub fn as_docx(&self) -> Option<&DocxExportOptions> {
        match self {
            Self::Docx { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_docx(self) -> Option<DocxExportOptions> {
        match self {
            Self::Docx { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_html(&self) -> bool {
        matches!(self, Self::Html { .. })
    }

    pub fn as_html(&self) -> Option<&HtmlExportOptions> {
        match self {
            Self::Html { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_html(self) -> Option<HtmlExportOptions> {
        match self {
            Self::Html { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_pdf(&self) -> bool {
        matches!(self, Self::Pdf { .. })
    }

    pub fn as_pdf(&self) -> Option<&PdfExportOptions> {
        match self {
            Self::Pdf { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_pdf(self) -> Option<PdfExportOptions> {
        match self {
            Self::Pdf { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_segmented_json(&self) -> bool {
        matches!(self, Self::SegmentedJson { .. })
    }

    pub fn as_segmented_json(&self) -> Option<&SegmentedJsonExportOptions> {
        match self {
            Self::SegmentedJson { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_segmented_json(self) -> Option<SegmentedJsonExportOptions> {
        match self {
            Self::SegmentedJson { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_srt(&self) -> bool {
        matches!(self, Self::Srt { .. })
    }

    pub fn as_srt(&self) -> Option<&SrtExportOptions> {
        match self {
            Self::Srt { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_srt(self) -> Option<SrtExportOptions> {
        match self {
            Self::Srt { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_txt(&self) -> bool {
        matches!(self, Self::Txt { .. })
    }

    pub fn as_txt(&self) -> Option<&TxtExportOptions> {
        match self {
            Self::Txt { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_txt(self) -> Option<TxtExportOptions> {
        match self {
            Self::Txt { data } => Some(data),
            _ => None,
        }
    }
}
