pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Init {
    #[serde(default)]
    pub conversation_id: String,
}

impl Init {
    pub fn builder() -> InitBuilder {
        <InitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InitBuilder {
    conversation_id: Option<String>,
}

impl InitBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Init`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](InitBuilder::conversation_id)
    pub fn build(self) -> Result<Init, BuildError> {
        Ok(Init {
            conversation_id: self
                .conversation_id
                .ok_or_else(|| BuildError::missing_field("conversation_id"))?,
        })
    }
}
