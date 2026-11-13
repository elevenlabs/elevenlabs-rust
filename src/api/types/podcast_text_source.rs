pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PodcastTextSource {
    /// The text to create the podcast from.
    pub text: String,
}

impl PodcastTextSource {
    pub fn builder() -> PodcastTextSourceBuilder {
        <PodcastTextSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodcastTextSourceBuilder {
    text: Option<String>,
}

impl PodcastTextSourceBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PodcastTextSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](PodcastTextSourceBuilder::text)
    pub fn build(self) -> Result<PodcastTextSource, BuildError> {
        Ok(PodcastTextSource {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
