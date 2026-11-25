pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyUpdateAudioNativeProjectContentV1AudioNativeProjectIdContentPost {
    /// Either txt or HTML input file containing the article content. HTML should be formatted as follows '&lt;html&gt;&lt;body&gt;&lt;div&gt;&lt;p&gt;Your content&lt;/p&gt;&lt;h5&gt;More of your content&lt;/h5&gt;&lt;p&gt;Some more of your content&lt;/p&gt;&lt;/div&gt;&lt;/body&gt;&lt;/html&gt;'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<Vec<u8>>,
    /// Whether to auto convert the project to audio or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_convert: Option<bool>,
    /// Whether to auto publish the new project snapshot after it's converted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_publish: Option<bool>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub file_file_name: Option<String>,
}

impl BodyUpdateAudioNativeProjectContentV1AudioNativeProjectIdContentPost {
    pub fn builder() -> BodyUpdateAudioNativeProjectContentV1AudioNativeProjectIdContentPostBuilder
    {
        <BodyUpdateAudioNativeProjectContentV1AudioNativeProjectIdContentPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpdateAudioNativeProjectContentV1AudioNativeProjectIdContentPostBuilder {
    file: Option<Vec<u8>>,
    auto_convert: Option<bool>,
    auto_publish: Option<bool>,
    file_file_name: Option<String>,
}

impl BodyUpdateAudioNativeProjectContentV1AudioNativeProjectIdContentPostBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn auto_convert(mut self, value: bool) -> Self {
        self.auto_convert = Some(value);
        self
    }

    pub fn auto_publish(mut self, value: bool) -> Self {
        self.auto_publish = Some(value);
        self
    }

    pub fn file_file_name(mut self, value: impl Into<String>) -> Self {
        self.file_file_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyUpdateAudioNativeProjectContentV1AudioNativeProjectIdContentPost`].
    pub fn build(
        self,
    ) -> Result<BodyUpdateAudioNativeProjectContentV1AudioNativeProjectIdContentPost, BuildError>
    {
        Ok(
            BodyUpdateAudioNativeProjectContentV1AudioNativeProjectIdContentPost {
                file: self.file,
                auto_convert: self.auto_convert,
                auto_publish: self.auto_publish,
                file_file_name: self.file_file_name,
            },
        )
    }
}

impl BodyUpdateAudioNativeProjectContentV1AudioNativeProjectIdContentPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        if let Some(value) = self.file {
            form = form.part("file", {
                let file_name = self.file_file_name.unwrap_or_else(|| "file".to_string());
                let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
                reqwest::multipart::Part::bytes(value)
                    .file_name(file_name)
                    .mime_str(mime.as_ref())
                    .expect("a guessed MIME type is valid")
            });
        }
        if let Some(value) = self.auto_convert {
            form = form.text("auto_convert", value.to_string());
        }
        if let Some(value) = self.auto_publish {
            form = form.text("auto_publish", value.to_string());
        }
        form
    }
}
