pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyCreatesAudioNativeEnabledProjectV1AudioNativePost {
    /// Project name.
    #[serde(default)]
    pub name: String,
    /// (Deprecated) Image URL used in the player. If not provided, default image set in the Player settings is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Author used in the player and inserted at the start of the uploaded article. If not provided, the default author set in the Player settings is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// Title used in the player and inserted at the top of the uploaded article. If not provided, the default title set in the Player settings is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// (Deprecated) Whether to use small player or not. If not provided, default value set in the Player settings is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small: Option<bool>,
    /// Text color used in the player. If not provided, default text color set in the Player settings is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    /// Background color used in the player. If not provided, default background color set in the Player settings is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// (Deprecated) Specifies for how many minutes to persist the session across page reloads. If not provided, default sessionization set in the Player settings is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub sessionization: Option<i64>,
    /// Voice ID used to voice the content. If not provided, default voice ID set in the Player settings is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// TTS Model ID used in the player. If not provided, default model ID set in the Player settings is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// Either txt or HTML input file containing the article content. HTML should be formatted as follows '&lt;html&gt;&lt;body&gt;&lt;div&gt;&lt;p&gt;Your content&lt;/p&gt;&lt;h3&gt;More of your content&lt;/h3&gt;&lt;p&gt;Some more of your content&lt;/p&gt;&lt;/div&gt;&lt;/body&gt;&lt;/html&gt;'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<Vec<u8>>,
    /// Whether to auto convert the project to audio or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_convert: Option<bool>,
    ///
    /// This parameter controls text normalization with four modes: 'auto', 'on', 'apply_english' and 'off'.
    /// When set to 'auto', the system will automatically decide whether to apply text normalization
    /// (e.g., spelling out numbers). With 'on', text normalization will always be applied, while
    /// with 'off', it will be skipped. 'apply_english' is the same as 'on' but will assume that text is in English.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_text_normalization:
        Option<BodyCreatePodcastV1StudioPodcastsPostApplyTextNormalization>,
    /// A list of pronunciation dictionary locators (pronunciation_dictionary_id, version_id) encoded as a list of JSON strings for pronunciation dictionaries to be applied to the text. A list of json encoded strings is required as adding projects may occur through formData as opposed to jsonBody. To specify multiple dictionaries use multiple --form lines in your curl, such as --form 'pronunciation_dictionary_locators="{\"pronunciation_dictionary_id\":\"Vmd4Zor6fplcA7WrINey\",\"version_id\":\"hRPaxjlTdR7wFMhV4w0b\"}"' --form 'pronunciation_dictionary_locators="{\"pronunciation_dictionary_id\":\"JzWtcGQMJ6bnlWwyMo7e\",\"version_id\":\"lbmwxiLu4q6txYxgdZqn\"}"'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators: Option<Vec<String>>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub file_file_name: Option<String>,
}

impl BodyCreatesAudioNativeEnabledProjectV1AudioNativePost {
    pub fn builder() -> BodyCreatesAudioNativeEnabledProjectV1AudioNativePostBuilder {
        <BodyCreatesAudioNativeEnabledProjectV1AudioNativePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreatesAudioNativeEnabledProjectV1AudioNativePostBuilder {
    name: Option<String>,
    image: Option<String>,
    author: Option<String>,
    title: Option<String>,
    small: Option<bool>,
    text_color: Option<String>,
    background_color: Option<String>,
    sessionization: Option<i64>,
    voice_id: Option<String>,
    model_id: Option<String>,
    file: Option<Vec<u8>>,
    auto_convert: Option<bool>,
    apply_text_normalization: Option<BodyCreatePodcastV1StudioPodcastsPostApplyTextNormalization>,
    pronunciation_dictionary_locators: Option<Vec<String>>,
    file_file_name: Option<String>,
}

impl BodyCreatesAudioNativeEnabledProjectV1AudioNativePostBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn image(mut self, value: impl Into<String>) -> Self {
        self.image = Some(value.into());
        self
    }

    pub fn author(mut self, value: impl Into<String>) -> Self {
        self.author = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn small(mut self, value: bool) -> Self {
        self.small = Some(value);
        self
    }

    pub fn text_color(mut self, value: impl Into<String>) -> Self {
        self.text_color = Some(value.into());
        self
    }

    pub fn background_color(mut self, value: impl Into<String>) -> Self {
        self.background_color = Some(value.into());
        self
    }

    pub fn sessionization(mut self, value: i64) -> Self {
        self.sessionization = Some(value);
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn auto_convert(mut self, value: bool) -> Self {
        self.auto_convert = Some(value);
        self
    }

    pub fn apply_text_normalization(
        mut self,
        value: BodyCreatePodcastV1StudioPodcastsPostApplyTextNormalization,
    ) -> Self {
        self.apply_text_normalization = Some(value);
        self
    }

    pub fn pronunciation_dictionary_locators(mut self, value: Vec<String>) -> Self {
        self.pronunciation_dictionary_locators = Some(value);
        self
    }

    pub fn file_file_name(mut self, value: impl Into<String>) -> Self {
        self.file_file_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyCreatesAudioNativeEnabledProjectV1AudioNativePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](BodyCreatesAudioNativeEnabledProjectV1AudioNativePostBuilder::name)
    pub fn build(
        self,
    ) -> Result<BodyCreatesAudioNativeEnabledProjectV1AudioNativePost, BuildError> {
        Ok(BodyCreatesAudioNativeEnabledProjectV1AudioNativePost {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            image: self.image,
            author: self.author,
            title: self.title,
            small: self.small,
            text_color: self.text_color,
            background_color: self.background_color,
            sessionization: self.sessionization,
            voice_id: self.voice_id,
            model_id: self.model_id,
            file: self.file,
            auto_convert: self.auto_convert,
            apply_text_normalization: self.apply_text_normalization,
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators,
            file_file_name: self.file_file_name,
        })
    }
}

impl BodyCreatesAudioNativeEnabledProjectV1AudioNativePost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.text("name", self.name.to_string());
        if let Some(value) = self.image {
            form = form.text("image", value.to_string());
        }
        if let Some(value) = self.author {
            form = form.text("author", value.to_string());
        }
        if let Some(value) = self.title {
            form = form.text("title", value.to_string());
        }
        if let Some(value) = self.small {
            form = form.text("small", value.to_string());
        }
        if let Some(value) = self.text_color {
            form = form.text("text_color", value.to_string());
        }
        if let Some(value) = self.background_color {
            form = form.text("background_color", value.to_string());
        }
        if let Some(value) = self.sessionization {
            form = form.text("sessionization", value.to_string());
        }
        if let Some(value) = self.voice_id {
            form = form.text("voice_id", value.to_string());
        }
        if let Some(value) = self.model_id {
            form = form.text("model_id", value.to_string());
        }
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
        if let Some(value) = self.apply_text_normalization {
            form = form.text("apply_text_normalization", value.to_string());
        }
        if let Some(value) = self.pronunciation_dictionary_locators {
            for part_value in value {
                form = form.text("pronunciation_dictionary_locators", part_value.to_string());
            }
        }
        form
    }
}
