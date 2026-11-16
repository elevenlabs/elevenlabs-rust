pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WhatsAppTemplateLocationParam {
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
    location: Option<WhatsAppTemplateLocationParamDetails>,
}

impl WhatsAppTemplateLocationParamBuilder {
    pub fn location(mut self, value: WhatsAppTemplateLocationParamDetails) -> Self {
        self.location = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppTemplateLocationParam`].
    /// This method will fail if any of the following fields are not set:
    /// - [`location`](WhatsAppTemplateLocationParamBuilder::location)
    pub fn build(self) -> Result<WhatsAppTemplateLocationParam, BuildError> {
        Ok(WhatsAppTemplateLocationParam {
            location: self
                .location
                .ok_or_else(|| BuildError::missing_field("location"))?,
        })
    }
}
