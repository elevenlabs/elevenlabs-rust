pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyUploadFileV1ConvaiConversationsConversationIdFilesPost {
    /// Image, PDF, or text file to upload
    #[serde(default)]
    pub file: Vec<u8>,
}

impl BodyUploadFileV1ConvaiConversationsConversationIdFilesPost {
    pub fn builder() -> BodyUploadFileV1ConvaiConversationsConversationIdFilesPostBuilder {
        <BodyUploadFileV1ConvaiConversationsConversationIdFilesPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUploadFileV1ConvaiConversationsConversationIdFilesPostBuilder {
    file: Option<Vec<u8>>,
}

impl BodyUploadFileV1ConvaiConversationsConversationIdFilesPostBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyUploadFileV1ConvaiConversationsConversationIdFilesPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](BodyUploadFileV1ConvaiConversationsConversationIdFilesPostBuilder::file)
    pub fn build(
        self,
    ) -> Result<BodyUploadFileV1ConvaiConversationsConversationIdFilesPost, BuildError> {
        Ok(BodyUploadFileV1ConvaiConversationsConversationIdFilesPost {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
        })
    }
}

impl BodyUploadFileV1ConvaiConversationsConversationIdFilesPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.part(
            "file",
            reqwest::multipart::Part::bytes(self.file)
                .file_name("file")
                .mime_str("application/octet-stream")
                .expect("application/octet-stream is a valid MIME type"),
        );
        form
    }
}
