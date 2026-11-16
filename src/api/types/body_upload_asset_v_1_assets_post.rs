pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyUploadAssetV1AssetsPost {
    /// The file to upload.
    #[serde(default)]
    pub asset: Vec<u8>,
    /// Display name for the asset.
    #[serde(default)]
    pub name: String,
}

impl BodyUploadAssetV1AssetsPost {
    pub fn builder() -> BodyUploadAssetV1AssetsPostBuilder {
        <BodyUploadAssetV1AssetsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUploadAssetV1AssetsPostBuilder {
    asset: Option<Vec<u8>>,
    name: Option<String>,
}

impl BodyUploadAssetV1AssetsPostBuilder {
    pub fn asset(mut self, value: Vec<u8>) -> Self {
        self.asset = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyUploadAssetV1AssetsPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset`](BodyUploadAssetV1AssetsPostBuilder::asset)
    /// - [`name`](BodyUploadAssetV1AssetsPostBuilder::name)
    pub fn build(self) -> Result<BodyUploadAssetV1AssetsPost, BuildError> {
        Ok(BodyUploadAssetV1AssetsPost {
            asset: self
                .asset
                .ok_or_else(|| BuildError::missing_field("asset"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

impl BodyUploadAssetV1AssetsPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.part(
            "asset",
            reqwest::multipart::Part::bytes(self.asset)
                .file_name("asset")
                .mime_str("application/octet-stream")
                .expect("application/octet-stream is a valid MIME type"),
        );
        form = form.text("name", self.name.to_string());
        form
    }
}
