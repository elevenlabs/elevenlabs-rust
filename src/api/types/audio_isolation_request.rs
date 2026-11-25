pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AudioIsolationRequest {
    /// The audio file from which vocals/speech will be isolated from.
    #[serde(default)]
    pub audio: Vec<u8>,
    /// The format of input audio. Options are 'pcm_s16le_16' or 'other' For `pcm_s16le_16`, the input audio must be 16-bit PCM at a 16kHz sample rate, single channel (mono), and little-endian byte order. Latency will be lower than with passing an encoded waveform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<AudioIsolationRequestFileFormat>,
    /// Optional preview image base64 for tracking this generation.
    #[serde(rename = "preview_b64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_b_64: Option<String>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub audio_file_name: Option<String>,
}

impl AudioIsolationRequest {
    pub fn builder() -> AudioIsolationRequestBuilder {
        <AudioIsolationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioIsolationRequestBuilder {
    audio: Option<Vec<u8>>,
    file_format: Option<AudioIsolationRequestFileFormat>,
    preview_b_64: Option<String>,
    audio_file_name: Option<String>,
}

impl AudioIsolationRequestBuilder {
    pub fn audio(mut self, value: Vec<u8>) -> Self {
        self.audio = Some(value);
        self
    }

    pub fn file_format(mut self, value: AudioIsolationRequestFileFormat) -> Self {
        self.file_format = Some(value);
        self
    }

    pub fn preview_b_64(mut self, value: impl Into<String>) -> Self {
        self.preview_b_64 = Some(value.into());
        self
    }

    pub fn audio_file_name(mut self, value: impl Into<String>) -> Self {
        self.audio_file_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioIsolationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio`](AudioIsolationRequestBuilder::audio)
    pub fn build(self) -> Result<AudioIsolationRequest, BuildError> {
        Ok(AudioIsolationRequest {
            audio: self
                .audio
                .ok_or_else(|| BuildError::missing_field("audio"))?,
            file_format: self.file_format,
            preview_b_64: self.preview_b_64,
            audio_file_name: self.audio_file_name,
        })
    }
}

impl AudioIsolationRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.part("audio", {
            let file_name = self.audio_file_name.unwrap_or_else(|| "audio".to_string());
            let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
            reqwest::multipart::Part::bytes(self.audio)
                .file_name(file_name)
                .mime_str(mime.as_ref())
                .expect("a guessed MIME type is valid")
        });
        if let Some(value) = self.file_format {
            form = form.text("file_format", value.to_string());
        }
        if let Some(value) = self.preview_b_64 {
            form = form.text("preview_b64", value.to_string());
        }
        form
    }
}
