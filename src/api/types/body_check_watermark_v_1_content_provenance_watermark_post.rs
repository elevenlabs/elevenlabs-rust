pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyCheckWatermarkV1ContentProvenanceWatermarkPost {
    /// Audio file to check for an embedded watermark.
    #[serde(default)]
    pub file: Vec<u8>,
    /// Watermark decoder version. Defaults to latest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub file_file_name: Option<String>,
}

impl BodyCheckWatermarkV1ContentProvenanceWatermarkPost {
    pub fn builder() -> BodyCheckWatermarkV1ContentProvenanceWatermarkPostBuilder {
        <BodyCheckWatermarkV1ContentProvenanceWatermarkPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCheckWatermarkV1ContentProvenanceWatermarkPostBuilder {
    file: Option<Vec<u8>>,
    version: Option<String>,
    file_file_name: Option<String>,
}

impl BodyCheckWatermarkV1ContentProvenanceWatermarkPostBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn file_file_name(mut self, value: impl Into<String>) -> Self {
        self.file_file_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyCheckWatermarkV1ContentProvenanceWatermarkPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](BodyCheckWatermarkV1ContentProvenanceWatermarkPostBuilder::file)
    pub fn build(self) -> Result<BodyCheckWatermarkV1ContentProvenanceWatermarkPost, BuildError> {
        Ok(BodyCheckWatermarkV1ContentProvenanceWatermarkPost {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            version: self.version,
            file_file_name: self.file_file_name,
        })
    }
}

impl BodyCheckWatermarkV1ContentProvenanceWatermarkPost {
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
        if let Some(value) = self.version {
            form = form.text("version", value.to_string());
        }
        form
    }
}
