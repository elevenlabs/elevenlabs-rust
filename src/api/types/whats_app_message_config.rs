pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WhatsAppMessageConfig {
    pub template: WhatsAppTemplate,
}

impl WhatsAppMessageConfig {
    pub fn builder() -> WhatsAppMessageConfigBuilder {
        <WhatsAppMessageConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppMessageConfigBuilder {
    template: Option<WhatsAppTemplate>,
}

impl WhatsAppMessageConfigBuilder {
    pub fn template(mut self, value: WhatsAppTemplate) -> Self {
        self.template = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppMessageConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`template`](WhatsAppMessageConfigBuilder::template)
    pub fn build(self) -> Result<WhatsAppMessageConfig, BuildError> {
        Ok(WhatsAppMessageConfig {
            template: self
                .template
                .ok_or_else(|| BuildError::missing_field("template"))?,
        })
    }
}
