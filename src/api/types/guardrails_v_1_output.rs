pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GuardrailsV1Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<GuardrailsV1InputVersion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<FocusGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_injection: Option<PromptInjectionGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentGuardrailOutput>,
    /// Container for custom guardrails, matching ModerationGuardrail pattern
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<CustomGuardrailOutput>,
}

impl GuardrailsV1Output {
    pub fn builder() -> GuardrailsV1OutputBuilder {
        <GuardrailsV1OutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GuardrailsV1OutputBuilder {
    version: Option<GuardrailsV1InputVersion>,
    focus: Option<FocusGuardrail>,
    prompt_injection: Option<PromptInjectionGuardrail>,
    content: Option<ContentGuardrailOutput>,
    custom: Option<CustomGuardrailOutput>,
}

impl GuardrailsV1OutputBuilder {
    pub fn version(mut self, value: GuardrailsV1InputVersion) -> Self {
        self.version = Some(value);
        self
    }

    pub fn focus(mut self, value: FocusGuardrail) -> Self {
        self.focus = Some(value);
        self
    }

    pub fn prompt_injection(mut self, value: PromptInjectionGuardrail) -> Self {
        self.prompt_injection = Some(value);
        self
    }

    pub fn content(mut self, value: ContentGuardrailOutput) -> Self {
        self.content = Some(value);
        self
    }

    pub fn custom(mut self, value: CustomGuardrailOutput) -> Self {
        self.custom = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GuardrailsV1Output`].
    pub fn build(self) -> Result<GuardrailsV1Output, BuildError> {
        Ok(GuardrailsV1Output {
            version: self.version,
            focus: self.focus,
            prompt_injection: self.prompt_injection,
            content: self.content,
            custom: self.custom,
        })
    }
}
