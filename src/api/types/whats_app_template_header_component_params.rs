pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WhatsAppTemplateHeaderComponentParams {
    #[serde(default)]
    pub parameters: Vec<ParametersItem>,
}

impl WhatsAppTemplateHeaderComponentParams {
    pub fn builder() -> WhatsAppTemplateHeaderComponentParamsBuilder {
        <WhatsAppTemplateHeaderComponentParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppTemplateHeaderComponentParamsBuilder {
    parameters: Option<Vec<ParametersItem>>,
}

impl WhatsAppTemplateHeaderComponentParamsBuilder {
    pub fn parameters(mut self, value: Vec<ParametersItem>) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppTemplateHeaderComponentParams`].
    /// This method will fail if any of the following fields are not set:
    /// - [`parameters`](WhatsAppTemplateHeaderComponentParamsBuilder::parameters)
    pub fn build(self) -> Result<WhatsAppTemplateHeaderComponentParams, BuildError> {
        Ok(WhatsAppTemplateHeaderComponentParams {
            parameters: self
                .parameters
                .ok_or_else(|| BuildError::missing_field("parameters"))?,
        })
    }
}
