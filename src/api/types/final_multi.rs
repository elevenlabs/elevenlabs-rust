pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinalMulti {
    /// Marks the end of the closing flush sequence.
    pub is_final: FinalIsFinal,
    /// Identifies which context this response belongs to. Only set on the multi-context endpoint; omitted (None) on the single-context endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

impl FinalMulti {
    pub fn builder() -> FinalMultiBuilder {
        <FinalMultiBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinalMultiBuilder {
    is_final: Option<FinalIsFinal>,
    context_id: Option<String>,
}

impl FinalMultiBuilder {
    pub fn is_final(mut self, value: FinalIsFinal) -> Self {
        self.is_final = Some(value);
        self
    }

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FinalMulti`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_final`](FinalMultiBuilder::is_final)
    pub fn build(self) -> Result<FinalMulti, BuildError> {
        Ok(FinalMulti {
            is_final: self
                .is_final
                .ok_or_else(|| BuildError::missing_field("is_final"))?,
            context_id: self.context_id,
        })
    }
}
