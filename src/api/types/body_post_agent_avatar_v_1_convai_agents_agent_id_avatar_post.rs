pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyPostAgentAvatarV1ConvaiAgentsAgentIdAvatarPost {
    /// An image file to be used as the agent's avatar.
    #[serde(default)]
    pub avatar_file: Vec<u8>,
}

impl BodyPostAgentAvatarV1ConvaiAgentsAgentIdAvatarPost {
    pub fn builder() -> BodyPostAgentAvatarV1ConvaiAgentsAgentIdAvatarPostBuilder {
        <BodyPostAgentAvatarV1ConvaiAgentsAgentIdAvatarPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyPostAgentAvatarV1ConvaiAgentsAgentIdAvatarPostBuilder {
    avatar_file: Option<Vec<u8>>,
}

impl BodyPostAgentAvatarV1ConvaiAgentsAgentIdAvatarPostBuilder {
    pub fn avatar_file(mut self, value: Vec<u8>) -> Self {
        self.avatar_file = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyPostAgentAvatarV1ConvaiAgentsAgentIdAvatarPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`avatar_file`](BodyPostAgentAvatarV1ConvaiAgentsAgentIdAvatarPostBuilder::avatar_file)
    pub fn build(self) -> Result<BodyPostAgentAvatarV1ConvaiAgentsAgentIdAvatarPost, BuildError> {
        Ok(BodyPostAgentAvatarV1ConvaiAgentsAgentIdAvatarPost {
            avatar_file: self
                .avatar_file
                .ok_or_else(|| BuildError::missing_field("avatar_file"))?,
        })
    }
}

impl BodyPostAgentAvatarV1ConvaiAgentsAgentIdAvatarPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.part(
            "avatar_file",
            reqwest::multipart::Part::bytes(self.avatar_file)
                .file_name("avatar_file")
                .mime_str("application/octet-stream")
                .expect("application/octet-stream is a valid MIME type"),
        );
        form
    }
}
