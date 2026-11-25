pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyPostAgentAvatarV1ConvaiAgentsAgentIdAvatarPost {
    /// An image file to be used as the agent's avatar.
    #[serde(default)]
    pub avatar_file: Vec<u8>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub avatar_file_file_name: Option<String>,
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
    avatar_file_file_name: Option<String>,
}

impl BodyPostAgentAvatarV1ConvaiAgentsAgentIdAvatarPostBuilder {
    pub fn avatar_file(mut self, value: Vec<u8>) -> Self {
        self.avatar_file = Some(value);
        self
    }

    pub fn avatar_file_file_name(mut self, value: impl Into<String>) -> Self {
        self.avatar_file_file_name = Some(value.into());
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
            avatar_file_file_name: self.avatar_file_file_name,
        })
    }
}

impl BodyPostAgentAvatarV1ConvaiAgentsAgentIdAvatarPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.part("avatar_file", {
            let file_name = self
                .avatar_file_file_name
                .unwrap_or_else(|| "avatar_file".to_string());
            let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
            reqwest::multipart::Part::bytes(self.avatar_file)
                .file_name(file_name)
                .mime_str(mime.as_ref())
                .expect("a guessed MIME type is valid")
        });
        form
    }
}
