pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrbAvatar {
    /// The first color of the avatar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_1: Option<String>,
    /// The second color of the avatar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_2: Option<String>,
}

impl OrbAvatar {
    pub fn builder() -> OrbAvatarBuilder {
        <OrbAvatarBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrbAvatarBuilder {
    color_1: Option<String>,
    color_2: Option<String>,
}

impl OrbAvatarBuilder {
    pub fn color_1(mut self, value: impl Into<String>) -> Self {
        self.color_1 = Some(value.into());
        self
    }

    pub fn color_2(mut self, value: impl Into<String>) -> Self {
        self.color_2 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrbAvatar`].
    pub fn build(self) -> Result<OrbAvatar, BuildError> {
        Ok(OrbAvatar {
            color_1: self.color_1,
            color_2: self.color_2,
        })
    }
}
