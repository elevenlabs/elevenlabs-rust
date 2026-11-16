pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WhatsAppTemplateBodyComponentParams {
    #[serde(default)]
    pub parameters: Vec<WhatsAppTemplateTextParam>,
}

impl WhatsAppTemplateBodyComponentParams {
    pub fn builder() -> WhatsAppTemplateBodyComponentParamsBuilder {
        <WhatsAppTemplateBodyComponentParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppTemplateBodyComponentParamsBuilder {
    parameters: Option<Vec<WhatsAppTemplateTextParam>>,
}

impl WhatsAppTemplateBodyComponentParamsBuilder {
    pub fn parameters(mut self, value: Vec<WhatsAppTemplateTextParam>) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppTemplateBodyComponentParams`].
    /// This method will fail if any of the following fields are not set:
    /// - [`parameters`](WhatsAppTemplateBodyComponentParamsBuilder::parameters)
    pub fn build(self) -> Result<WhatsAppTemplateBodyComponentParams, BuildError> {
        Ok(WhatsAppTemplateBodyComponentParams {
            parameters: self
                .parameters
                .ok_or_else(|| BuildError::missing_field("parameters"))?,
        })
    }
}
