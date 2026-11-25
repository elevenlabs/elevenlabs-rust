pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyUploadFileV1ConvaiConversationsConversationIdFilesPost {
    /// Image, PDF, or text file to upload
    #[serde(default)]
    pub file: Vec<u8>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub file_file_name: Option<String>,
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
    file_file_name: Option<String>,
}

impl BodyUploadFileV1ConvaiConversationsConversationIdFilesPostBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn file_file_name(mut self, value: impl Into<String>) -> Self {
        self.file_file_name = Some(value.into());
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
            file_file_name: self.file_file_name,
        })
    }
}

impl BodyUploadFileV1ConvaiConversationsConversationIdFilesPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.part("file", {
            let file_name = self.file_file_name.unwrap_or_else(|| "file".to_string());
            let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
            reqwest::multipart::Part::bytes(self.file)
                .file_name(file_name)
                .mime_str(mime.as_ref())
                .expect("a guessed MIME type is valid")
        });
        form
    }
}
