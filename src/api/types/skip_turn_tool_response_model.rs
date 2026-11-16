pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SkipTurnToolResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AgentSuccessfulResponseExampleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl SkipTurnToolResponseModel {
    pub fn builder() -> SkipTurnToolResponseModelBuilder {
        <SkipTurnToolResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SkipTurnToolResponseModelBuilder {
    status: Option<AgentSuccessfulResponseExampleType>,
    reason: Option<String>,
}

impl SkipTurnToolResponseModelBuilder {
    pub fn status(mut self, value: AgentSuccessfulResponseExampleType) -> Self {
        self.status = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SkipTurnToolResponseModel`].
    pub fn build(self) -> Result<SkipTurnToolResponseModel, BuildError> {
        Ok(SkipTurnToolResponseModel {
            status: self.status,
            reason: self.reason,
        })
    }
}
