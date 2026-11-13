pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostWorkspaceSecretRequest {
    pub r#type: PostWorkspaceSecretRequestType,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl PostWorkspaceSecretRequest {
    pub fn builder() -> PostWorkspaceSecretRequestBuilder {
        <PostWorkspaceSecretRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostWorkspaceSecretRequestBuilder {
    r#type: Option<PostWorkspaceSecretRequestType>,
    name: Option<String>,
    value: Option<String>,
}

impl PostWorkspaceSecretRequestBuilder {
    pub fn r#type(mut self, value: PostWorkspaceSecretRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostWorkspaceSecretRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PostWorkspaceSecretRequestBuilder::type)
    /// - [`name`](PostWorkspaceSecretRequestBuilder::name)
    /// - [`value`](PostWorkspaceSecretRequestBuilder::value)
    pub fn build(self) -> Result<PostWorkspaceSecretRequest, BuildError> {
        Ok(PostWorkspaceSecretRequest {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
