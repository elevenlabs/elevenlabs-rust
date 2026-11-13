pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FinalOutput {
    /// Indicates the generation is complete. When true, audio is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_final: Option<FinalOutputIsFinal>,
}

impl FinalOutput {
    pub fn builder() -> FinalOutputBuilder {
        <FinalOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinalOutputBuilder {
    is_final: Option<FinalOutputIsFinal>,
}

impl FinalOutputBuilder {
    pub fn is_final(mut self, value: FinalOutputIsFinal) -> Self {
        self.is_final = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FinalOutput`].
    pub fn build(self) -> Result<FinalOutput, BuildError> {
        Ok(FinalOutput {
            is_final: self.is_final,
        })
    }
}
