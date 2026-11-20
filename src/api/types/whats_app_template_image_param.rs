pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WhatsAppTemplateImageParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ImageSchemaType>,
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
    r#type: Option<ImageSchemaType>,
    image: Option<WhatsAppTemplateImageParamDetails>,
}

impl WhatsAppTemplateImageParamBuilder {
    pub fn r#type(mut self, value: ImageSchemaType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn image(mut self, value: WhatsAppTemplateImageParamDetails) -> Self {
        self.image = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppTemplateImageParam`].
    /// This method will fail if any of the following fields are not set:
    /// - [`image`](WhatsAppTemplateImageParamBuilder::image)
    pub fn build(self) -> Result<WhatsAppTemplateImageParam, BuildError> {
        Ok(WhatsAppTemplateImageParam {
            r#type: self.r#type,
            image: self
                .image
                .ok_or_else(|| BuildError::missing_field("image"))?,
        })
    }
}
