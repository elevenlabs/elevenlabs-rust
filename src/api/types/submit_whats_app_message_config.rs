pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubmitWhatsAppMessageConfig {
    /// Names the approved Meta template to send. The server loads the template
    /// itself, so the client never supplies its components.
    pub template: SubmitWhatsAppTemplateRef,
}

impl SubmitWhatsAppMessageConfig {
    pub fn builder() -> SubmitWhatsAppMessageConfigBuilder {
        <SubmitWhatsAppMessageConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitWhatsAppMessageConfigBuilder {
    template: Option<SubmitWhatsAppTemplateRef>,
}

impl SubmitWhatsAppMessageConfigBuilder {
    pub fn template(mut self, value: SubmitWhatsAppTemplateRef) -> Self {
        self.template = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitWhatsAppMessageConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`template`](SubmitWhatsAppMessageConfigBuilder::template)
    pub fn build(self) -> Result<SubmitWhatsAppMessageConfig, BuildError> {
        Ok(SubmitWhatsAppMessageConfig {
            template: self
                .template
                .ok_or_else(|| BuildError::missing_field("template"))?,
        })
    }
}
