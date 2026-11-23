pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserAudioChunk {
    /// Base64-encoded audio data from the user's microphone.
    pub user_audio_chunk: String,
}

impl UserAudioChunk {
    pub fn builder() -> UserAudioChunkBuilder {
        <UserAudioChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserAudioChunkBuilder {
    user_audio_chunk: Option<String>,
}

impl UserAudioChunkBuilder {
    pub fn user_audio_chunk(mut self, value: impl Into<String>) -> Self {
        self.user_audio_chunk = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserAudioChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_audio_chunk`](UserAudioChunkBuilder::user_audio_chunk)
    pub fn build(self) -> Result<UserAudioChunk, BuildError> {
        Ok(UserAudioChunk {
            user_audio_chunk: self
                .user_audio_chunk
                .ok_or_else(|| BuildError::missing_field("user_audio_chunk"))?,
        })
    }
}
