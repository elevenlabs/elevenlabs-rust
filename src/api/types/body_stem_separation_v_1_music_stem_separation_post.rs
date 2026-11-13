pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyStemSeparationV1MusicStemSeparationPost {
    /// The audio file to separate into stems.
    #[serde(default)]
    pub file: Vec<u8>,
    /// The id of the stem variation to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stem_variation_id: Option<StemVariationId>,
    /// Whether to sign the generated song with C2PA. Applicable only for mp3 files.
    #[serde(rename = "sign_with_c2pa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_with_c_2_pa: Option<bool>,
}

impl BodyStemSeparationV1MusicStemSeparationPost {
    pub fn builder() -> BodyStemSeparationV1MusicStemSeparationPostBuilder {
        <BodyStemSeparationV1MusicStemSeparationPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyStemSeparationV1MusicStemSeparationPostBuilder {
    file: Option<Vec<u8>>,
    stem_variation_id: Option<StemVariationId>,
    sign_with_c_2_pa: Option<bool>,
}

impl BodyStemSeparationV1MusicStemSeparationPostBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn stem_variation_id(mut self, value: StemVariationId) -> Self {
        self.stem_variation_id = Some(value);
        self
    }

    pub fn sign_with_c_2_pa(mut self, value: bool) -> Self {
        self.sign_with_c_2_pa = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyStemSeparationV1MusicStemSeparationPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](BodyStemSeparationV1MusicStemSeparationPostBuilder::file)
    pub fn build(self) -> Result<BodyStemSeparationV1MusicStemSeparationPost, BuildError> {
        Ok(BodyStemSeparationV1MusicStemSeparationPost {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            stem_variation_id: self.stem_variation_id,
            sign_with_c_2_pa: self.sign_with_c_2_pa,
        })
    }
}

impl BodyStemSeparationV1MusicStemSeparationPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.part(
            "file",
            reqwest::multipart::Part::bytes(self.file)
                .file_name("file")
                .mime_str("application/octet-stream")
                .expect("application/octet-stream is a valid MIME type"),
        );
        if let Some(value) = self.stem_variation_id {
            form = form.text("stem_variation_id", value.to_string());
        }
        if let Some(value) = self.sign_with_c_2_pa {
            form = form.text("sign_with_c2pa", value.to_string());
        }
        form
    }
}
