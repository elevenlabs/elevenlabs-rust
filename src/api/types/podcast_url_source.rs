pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PodcastUrlSource {
    /// The URL to create the podcast from.
    pub url: String,
}

impl PodcastUrlSource {
    pub fn builder() -> PodcastUrlSourceBuilder {
        <PodcastUrlSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodcastUrlSourceBuilder {
    url: Option<String>,
}

impl PodcastUrlSourceBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PodcastUrlSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](PodcastUrlSourceBuilder::url)
    pub fn build(self) -> Result<PodcastUrlSource, BuildError> {
        Ok(PodcastUrlSource {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
