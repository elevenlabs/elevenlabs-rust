pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SpeechToSpeechStreamRequest {
    /// The audio file which holds the content and emotion that will control the generated speech.
    #[serde(default)]
    pub audio: Vec<u8>,
    /// Identifier of the model that will be used, you can query them using GET /v1/models. The model needs to have support for speech to speech, you can check this using the can_do_voice_conversion property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// Voice settings overriding stored settings for the given voice. They are applied only on the given request. Needs to be send as a JSON encoded string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<String>,
    /// If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same seed and parameters should return the same result. Determinism is not guaranteed. Must be integer between 0 and 4294967295.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub seed: Option<i64>,
    /// If set, will remove the background noise from your audio input using our audio isolation model. Only applies to Voice Changer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_background_noise: Option<bool>,
    /// The format of input audio. Options are 'pcm_s16le_16' or 'other' For `pcm_s16le_16`, the input audio must be 16-bit PCM at a 16kHz sample rate, single channel (mono), and little-endian byte order. Latency will be lower than with passing an encoded waveform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<AudioIsolationRequestFileFormat>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub audio_file_name: Option<String>,
}

impl SpeechToSpeechStreamRequest {
    pub fn builder() -> SpeechToSpeechStreamRequestBuilder {
        <SpeechToSpeechStreamRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeechToSpeechStreamRequestBuilder {
    audio: Option<Vec<u8>>,
    model_id: Option<String>,
    voice_settings: Option<String>,
    seed: Option<i64>,
    remove_background_noise: Option<bool>,
    file_format: Option<AudioIsolationRequestFileFormat>,
    audio_file_name: Option<String>,
}

impl SpeechToSpeechStreamRequestBuilder {
    pub fn audio(mut self, value: Vec<u8>) -> Self {
        self.audio = Some(value);
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn voice_settings(mut self, value: impl Into<String>) -> Self {
        self.voice_settings = Some(value.into());
        self
    }

    pub fn seed(mut self, value: i64) -> Self {
        self.seed = Some(value);
        self
    }

    pub fn remove_background_noise(mut self, value: bool) -> Self {
        self.remove_background_noise = Some(value);
        self
    }

    pub fn file_format(mut self, value: AudioIsolationRequestFileFormat) -> Self {
        self.file_format = Some(value);
        self
    }

    pub fn audio_file_name(mut self, value: impl Into<String>) -> Self {
        self.audio_file_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpeechToSpeechStreamRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio`](SpeechToSpeechStreamRequestBuilder::audio)
    pub fn build(self) -> Result<SpeechToSpeechStreamRequest, BuildError> {
        Ok(SpeechToSpeechStreamRequest {
            audio: self
                .audio
                .ok_or_else(|| BuildError::missing_field("audio"))?,
            model_id: self.model_id,
            voice_settings: self.voice_settings,
            seed: self.seed,
            remove_background_noise: self.remove_background_noise,
            file_format: self.file_format,
            audio_file_name: self.audio_file_name,
        })
    }
}

impl SpeechToSpeechStreamRequest {
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
        if let Some(value) = self.model_id {
            form = form.text("model_id", value.to_string());
        }
        if let Some(value) = self.voice_settings {
            form = form.text("voice_settings", value.to_string());
        }
        if let Some(value) = self.seed {
            form = form.text("seed", value.to_string());
        }
        if let Some(value) = self.remove_background_noise {
            form = form.text("remove_background_noise", value.to_string());
        }
        if let Some(value) = self.file_format {
            form = form.text("file_format", value.to_string());
        }
        form
    }
}
