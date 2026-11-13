pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WhatsAppTemplateButtonComponentParams {
    #[serde(default)]
    pub parameters: Vec<WhatsAppTemplateTextParam>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub index: i64,
    #[serde(default)]
    pub sub_type: String,
}

impl WhatsAppTemplateButtonComponentParams {
    pub fn builder() -> WhatsAppTemplateButtonComponentParamsBuilder {
        <WhatsAppTemplateButtonComponentParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppTemplateButtonComponentParamsBuilder {
    parameters: Option<Vec<WhatsAppTemplateTextParam>>,
    index: Option<i64>,
    sub_type: Option<String>,
}

impl WhatsAppTemplateButtonComponentParamsBuilder {
    pub fn parameters(mut self, value: Vec<WhatsAppTemplateTextParam>) -> Self {
        self.parameters = Some(value);
        self
    }

    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn sub_type(mut self, value: impl Into<String>) -> Self {
        self.sub_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppTemplateButtonComponentParams`].
    /// This method will fail if any of the following fields are not set:
    /// - [`parameters`](WhatsAppTemplateButtonComponentParamsBuilder::parameters)
    /// - [`index`](WhatsAppTemplateButtonComponentParamsBuilder::index)
    /// - [`sub_type`](WhatsAppTemplateButtonComponentParamsBuilder::sub_type)
    pub fn build(self) -> Result<WhatsAppTemplateButtonComponentParams, BuildError> {
        Ok(WhatsAppTemplateButtonComponentParams {
            parameters: self
                .parameters
                .ok_or_else(|| BuildError::missing_field("parameters"))?,
            index: self
                .index
                .ok_or_else(|| BuildError::missing_field("index"))?,
            sub_type: self
                .sub_type
                .ok_or_else(|| BuildError::missing_field("sub_type"))?,
        })
    }
}
