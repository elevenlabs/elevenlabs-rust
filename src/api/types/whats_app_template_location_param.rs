pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WhatsAppTemplateLocationParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<WhatsAppTemplateLocationParamType>,
    pub location: WhatsAppTemplateLocationParamDetails,
}

impl WhatsAppTemplateLocationParam {
    pub fn builder() -> WhatsAppTemplateLocationParamBuilder {
        <WhatsAppTemplateLocationParamBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppTemplateLocationParamBuilder {
    r#type: Option<WhatsAppTemplateLocationParamType>,
    location: Option<WhatsAppTemplateLocationParamDetails>,
}

impl WhatsAppTemplateLocationParamBuilder {
    pub fn r#type(mut self, value: WhatsAppTemplateLocationParamType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn location(mut self, value: WhatsAppTemplateLocationParamDetails) -> Self {
        self.location = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppTemplateLocationParam`].
    /// This method will fail if any of the following fields are not set:
    /// - [`location`](WhatsAppTemplateLocationParamBuilder::location)
    pub fn build(self) -> Result<WhatsAppTemplateLocationParam, BuildError> {
        Ok(WhatsAppTemplateLocationParam {
            r#type: self.r#type,
            location: self
                .location
                .ok_or_else(|| BuildError::missing_field("location"))?,
        })
    }
}
