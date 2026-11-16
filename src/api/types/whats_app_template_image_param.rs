pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WhatsAppTemplateImageParam {
    pub image: WhatsAppTemplateImageParamDetails,
}

impl WhatsAppTemplateImageParam {
    pub fn builder() -> WhatsAppTemplateImageParamBuilder {
        <WhatsAppTemplateImageParamBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppTemplateImageParamBuilder {
    image: Option<WhatsAppTemplateImageParamDetails>,
}

impl WhatsAppTemplateImageParamBuilder {
    pub fn image(mut self, value: WhatsAppTemplateImageParamDetails) -> Self {
        self.image = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppTemplateImageParam`].
    /// This method will fail if any of the following fields are not set:
    /// - [`image`](WhatsAppTemplateImageParamBuilder::image)
    pub fn build(self) -> Result<WhatsAppTemplateImageParam, BuildError> {
        Ok(WhatsAppTemplateImageParam {
            image: self
                .image
                .ok_or_else(|| BuildError::missing_field("image"))?,
        })
    }
}
