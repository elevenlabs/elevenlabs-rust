pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSimilarVoicesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_file: Option<Vec<u8>>,
    /// Threshold for voice similarity between provided sample and library voices. Values range from 0 to 2. The smaller the value the more similar voices will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub similarity_threshold: Option<f64>,
    /// Number of most similar voices to return. If similarity_threshold is provided, less than this number of voices may be returned. Values range from 1 to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub top_k: Option<i64>,
}

impl ListSimilarVoicesRequest {
    pub fn builder() -> ListSimilarVoicesRequestBuilder {
        <ListSimilarVoicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSimilarVoicesRequestBuilder {
    audio_file: Option<Vec<u8>>,
    similarity_threshold: Option<f64>,
    top_k: Option<i64>,
}

impl ListSimilarVoicesRequestBuilder {
    pub fn audio_file(mut self, value: Vec<u8>) -> Self {
        self.audio_file = Some(value);
        self
    }

    pub fn similarity_threshold(mut self, value: f64) -> Self {
        self.similarity_threshold = Some(value);
        self
    }

    pub fn top_k(mut self, value: i64) -> Self {
        self.top_k = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSimilarVoicesRequest`].
    pub fn build(self) -> Result<ListSimilarVoicesRequest, BuildError> {
        Ok(ListSimilarVoicesRequest {
            audio_file: self.audio_file,
            similarity_threshold: self.similarity_threshold,
            top_k: self.top_k,
        })
    }
}

impl ListSimilarVoicesRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        if let Some(value) = self.audio_file {
            form = form.part(
                "audio_file",
                reqwest::multipart::Part::bytes(value)
                    .file_name("audio_file")
                    .mime_str("application/octet-stream")
                    .expect("application/octet-stream is a valid MIME type"),
            );
        }
        if let Some(value) = self.similarity_threshold {
            form = form.text("similarity_threshold", value.to_string());
        }
        if let Some(value) = self.top_k {
            form = form.text("top_k", value.to_string());
        }
        form
    }
}
