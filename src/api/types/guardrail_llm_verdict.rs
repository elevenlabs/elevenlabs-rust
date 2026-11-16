pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GuardrailLlmVerdict {
    /// The LLM's explanation for its decision.
    #[serde(default)]
    pub rationale: String,
}

impl GuardrailLlmVerdict {
    pub fn builder() -> GuardrailLlmVerdictBuilder {
        <GuardrailLlmVerdictBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GuardrailLlmVerdictBuilder {
    rationale: Option<String>,
}

impl GuardrailLlmVerdictBuilder {
    pub fn rationale(mut self, value: impl Into<String>) -> Self {
        self.rationale = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GuardrailLlmVerdict`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rationale`](GuardrailLlmVerdictBuilder::rationale)
    pub fn build(self) -> Result<GuardrailLlmVerdict, BuildError> {
        Ok(GuardrailLlmVerdict {
            rationale: self
                .rationale
                .ok_or_else(|| BuildError::missing_field("rationale"))?,
        })
    }
}
