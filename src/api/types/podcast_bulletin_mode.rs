pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PodcastBulletinMode {
    pub bulletin: PodcastBulletinModeData,
}

impl PodcastBulletinMode {
    pub fn builder() -> PodcastBulletinModeBuilder {
        <PodcastBulletinModeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodcastBulletinModeBuilder {
    bulletin: Option<PodcastBulletinModeData>,
}

impl PodcastBulletinModeBuilder {
    pub fn bulletin(mut self, value: PodcastBulletinModeData) -> Self {
        self.bulletin = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PodcastBulletinMode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bulletin`](PodcastBulletinModeBuilder::bulletin)
    pub fn build(self) -> Result<PodcastBulletinMode, BuildError> {
        Ok(PodcastBulletinMode {
            bulletin: self
                .bulletin
                .ok_or_else(|| BuildError::missing_field("bulletin"))?,
        })
    }
}
