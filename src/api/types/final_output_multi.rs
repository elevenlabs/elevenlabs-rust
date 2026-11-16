pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FinalOutputMulti {
    /// Indicates the generation is complete. When true, audio is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_final: Option<FinalOutputIsFinal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

impl FinalOutputMulti {
    pub fn builder() -> FinalOutputMultiBuilder {
        <FinalOutputMultiBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinalOutputMultiBuilder {
    is_final: Option<FinalOutputIsFinal>,
    context_id: Option<String>,
}

impl FinalOutputMultiBuilder {
    pub fn is_final(mut self, value: FinalOutputIsFinal) -> Self {
        self.is_final = Some(value);
        self
    }

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FinalOutputMulti`].
    pub fn build(self) -> Result<FinalOutputMulti, BuildError> {
        Ok(FinalOutputMulti {
            is_final: self.is_final,
            context_id: self.context_id,
        })
    }
}
