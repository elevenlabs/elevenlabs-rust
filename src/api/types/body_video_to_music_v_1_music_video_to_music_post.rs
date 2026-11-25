pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyVideoToMusicV1MusicVideoToMusicPost {
    ///
    /// One or more video files sent via FormData array (multipart/form-data). They will be combined into one codec in order.
    /// A maximum of 10 videos is allowed, where the total size of the combined video is limited to 200MB.
    /// In total, the video can be up to 600 seconds long. Note that combining multiple videos may increase the request duration significantly. If possible, combine the videos beforehand.
    ///
    #[serde(default)]
    pub videos: Vec<Vec<u8>>,
    /// Optional text description of the music you want. A maximum of 4100 characters is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional list of style tags (e.g. ['upbeat', 'cinematic']). A maximum of 10 tags is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<MusicModelId>,
    /// When true, incorporate SFX cues from the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_sfx: Option<bool>,
    /// When true, return a ZIP with separate music and SFX audio tracks instead of a single mixed file. Use this for more flexibility to layer the tracks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separate_tracks: Option<bool>,
    /// When true, analyze the video's audio track and use its genres, BPM, instrumentation, mood, and arc as reference characteristics for music generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_audio_as_reference: Option<bool>,
    /// Whether to sign the generated song with C2PA. Applicable only for mp3 files.
    #[serde(rename = "sign_with_c2pa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_with_c_2_pa: Option<bool>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub videos_file_names: Option<Vec<Option<String>>>,
}

impl BodyVideoToMusicV1MusicVideoToMusicPost {
    pub fn builder() -> BodyVideoToMusicV1MusicVideoToMusicPostBuilder {
        <BodyVideoToMusicV1MusicVideoToMusicPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyVideoToMusicV1MusicVideoToMusicPostBuilder {
    videos: Option<Vec<Vec<u8>>>,
    description: Option<String>,
    tags: Option<Vec<String>>,
    model_id: Option<MusicModelId>,
    add_sfx: Option<bool>,
    separate_tracks: Option<bool>,
    use_audio_as_reference: Option<bool>,
    sign_with_c_2_pa: Option<bool>,
    videos_file_names: Option<Vec<Option<String>>>,
}

impl BodyVideoToMusicV1MusicVideoToMusicPostBuilder {
    pub fn videos(mut self, value: Vec<Vec<u8>>) -> Self {
        self.videos = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn model_id(mut self, value: MusicModelId) -> Self {
        self.model_id = Some(value);
        self
    }

    pub fn add_sfx(mut self, value: bool) -> Self {
        self.add_sfx = Some(value);
        self
    }

    pub fn separate_tracks(mut self, value: bool) -> Self {
        self.separate_tracks = Some(value);
        self
    }

    pub fn use_audio_as_reference(mut self, value: bool) -> Self {
        self.use_audio_as_reference = Some(value);
        self
    }

    pub fn sign_with_c_2_pa(mut self, value: bool) -> Self {
        self.sign_with_c_2_pa = Some(value);
        self
    }

    pub fn videos_file_names(mut self, value: Vec<Option<String>>) -> Self {
        self.videos_file_names = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyVideoToMusicV1MusicVideoToMusicPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`videos`](BodyVideoToMusicV1MusicVideoToMusicPostBuilder::videos)
    pub fn build(self) -> Result<BodyVideoToMusicV1MusicVideoToMusicPost, BuildError> {
        Ok(BodyVideoToMusicV1MusicVideoToMusicPost {
            videos: self
                .videos
                .ok_or_else(|| BuildError::missing_field("videos"))?,
            description: self.description,
            tags: self.tags,
            model_id: self.model_id,
            add_sfx: self.add_sfx,
            separate_tracks: self.separate_tracks,
            use_audio_as_reference: self.use_audio_as_reference,
            sign_with_c_2_pa: self.sign_with_c_2_pa,
            videos_file_names: self.videos_file_names,
        })
    }
}

impl BodyVideoToMusicV1MusicVideoToMusicPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        let mut videos_file_names_iter = self.videos_file_names.unwrap_or_default().into_iter();
        for part_value in self.videos {
            form = form.part("videos", {
                let file_name = videos_file_names_iter
                    .next()
                    .flatten()
                    .unwrap_or_else(|| "videos".to_string());
                let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
                reqwest::multipart::Part::bytes(part_value)
                    .file_name(file_name)
                    .mime_str(mime.as_ref())
                    .expect("a guessed MIME type is valid")
            });
        }
        if let Some(value) = self.description {
            form = form.text("description", value.to_string());
        }
        if let Some(value) = self.tags {
            for part_value in value {
                form = form.text("tags", part_value.to_string());
            }
        }
        if let Some(value) = self.model_id {
            form = form.text("model_id", value.to_string());
        }
        if let Some(value) = self.add_sfx {
            form = form.text("add_sfx", value.to_string());
        }
        if let Some(value) = self.separate_tracks {
            form = form.text("separate_tracks", value.to_string());
        }
        if let Some(value) = self.use_audio_as_reference {
            form = form.text("use_audio_as_reference", value.to_string());
        }
        if let Some(value) = self.sign_with_c_2_pa {
            form = form.text("sign_with_c2pa", value.to_string());
        }
        form
    }
}
