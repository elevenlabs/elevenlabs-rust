pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WhatsAppTemplateDocumentParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<WhatsAppTemplateDocumentParamType>,
    pub document: WhatsAppTemplateDocumentParamDetails,
}

impl WhatsAppTemplateDocumentParam {
    pub fn builder() -> WhatsAppTemplateDocumentParamBuilder {
        <WhatsAppTemplateDocumentParamBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppTemplateDocumentParamBuilder {
    r#type: Option<WhatsAppTemplateDocumentParamType>,
    document: Option<WhatsAppTemplateDocumentParamDetails>,
}

impl WhatsAppTemplateDocumentParamBuilder {
    pub fn r#type(mut self, value: WhatsAppTemplateDocumentParamType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn document(mut self, value: WhatsAppTemplateDocumentParamDetails) -> Self {
        self.document = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppTemplateDocumentParam`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document`](WhatsAppTemplateDocumentParamBuilder::document)
    pub fn build(self) -> Result<WhatsAppTemplateDocumentParam, BuildError> {
        Ok(WhatsAppTemplateDocumentParam {
            r#type: self.r#type,
            document: self
                .document
                .ok_or_else(|| BuildError::missing_field("document"))?,
        })
    }
}
