pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectImageResponseModel {
    #[serde(default)]
    pub image_id: String,
    #[serde(default)]
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_signed_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ProjectImageResponseModelSource>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub file_size_bytes: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub width: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub height: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_id: Option<String>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub offset_ms: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub duration_ms: i64,
    #[serde(default)]
    pub order: String,
    /// Defines asset positioning and transformation on canvas.
    #[serde(default)]
    pub canvas_placement: CanvasPlacement,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<ClipAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub opacity: Option<f64>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub created_at_ms: i64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub updated_at_ms: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_snapshot_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_asset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_platform_asset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_task: Option<PendingClipTask>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<ImageAnalysis>,
}

impl ProjectImageResponseModel {
    pub fn builder() -> ProjectImageResponseModelBuilder {
        <ProjectImageResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectImageResponseModelBuilder {
    image_id: Option<String>,
    filename: Option<String>,
    signed_url: Option<String>,
    thumbnail_signed_url: Option<String>,
    source: Option<ProjectImageResponseModelSource>,
    file_size_bytes: Option<i64>,
    width: Option<i64>,
    height: Option<i64>,
    track_id: Option<String>,
    offset_ms: Option<i64>,
    duration_ms: Option<i64>,
    order: Option<String>,
    canvas_placement: Option<CanvasPlacement>,
    animation: Option<ClipAnimation>,
    opacity: Option<f64>,
    created_at_ms: Option<i64>,
    updated_at_ms: Option<i64>,
    current_snapshot_id: Option<String>,
    source_asset_id: Option<String>,
    source_platform_asset_id: Option<String>,
    error: Option<String>,
    pending_task: Option<PendingClipTask>,
    analysis: Option<ImageAnalysis>,
}

impl ProjectImageResponseModelBuilder {
    pub fn image_id(mut self, value: impl Into<String>) -> Self {
        self.image_id = Some(value.into());
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    pub fn signed_url(mut self, value: impl Into<String>) -> Self {
        self.signed_url = Some(value.into());
        self
    }

    pub fn thumbnail_signed_url(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_signed_url = Some(value.into());
        self
    }

    pub fn source(mut self, value: ProjectImageResponseModelSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn file_size_bytes(mut self, value: i64) -> Self {
        self.file_size_bytes = Some(value);
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn track_id(mut self, value: impl Into<String>) -> Self {
        self.track_id = Some(value.into());
        self
    }

    pub fn offset_ms(mut self, value: i64) -> Self {
        self.offset_ms = Some(value);
        self
    }

    pub fn duration_ms(mut self, value: i64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn order(mut self, value: impl Into<String>) -> Self {
        self.order = Some(value.into());
        self
    }

    pub fn canvas_placement(mut self, value: CanvasPlacement) -> Self {
        self.canvas_placement = Some(value);
        self
    }

    pub fn animation(mut self, value: ClipAnimation) -> Self {
        self.animation = Some(value);
        self
    }

    pub fn opacity(mut self, value: f64) -> Self {
        self.opacity = Some(value);
        self
    }

    pub fn created_at_ms(mut self, value: i64) -> Self {
        self.created_at_ms = Some(value);
        self
    }

    pub fn updated_at_ms(mut self, value: i64) -> Self {
        self.updated_at_ms = Some(value);
        self
    }

    pub fn current_snapshot_id(mut self, value: impl Into<String>) -> Self {
        self.current_snapshot_id = Some(value.into());
        self
    }

    pub fn source_asset_id(mut self, value: impl Into<String>) -> Self {
        self.source_asset_id = Some(value.into());
        self
    }

    pub fn source_platform_asset_id(mut self, value: impl Into<String>) -> Self {
        self.source_platform_asset_id = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn pending_task(mut self, value: PendingClipTask) -> Self {
        self.pending_task = Some(value);
        self
    }

    pub fn analysis(mut self, value: ImageAnalysis) -> Self {
        self.analysis = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectImageResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`image_id`](ProjectImageResponseModelBuilder::image_id)
    /// - [`filename`](ProjectImageResponseModelBuilder::filename)
    /// - [`file_size_bytes`](ProjectImageResponseModelBuilder::file_size_bytes)
    /// - [`width`](ProjectImageResponseModelBuilder::width)
    /// - [`height`](ProjectImageResponseModelBuilder::height)
    /// - [`offset_ms`](ProjectImageResponseModelBuilder::offset_ms)
    /// - [`duration_ms`](ProjectImageResponseModelBuilder::duration_ms)
    /// - [`order`](ProjectImageResponseModelBuilder::order)
    /// - [`canvas_placement`](ProjectImageResponseModelBuilder::canvas_placement)
    /// - [`created_at_ms`](ProjectImageResponseModelBuilder::created_at_ms)
    /// - [`updated_at_ms`](ProjectImageResponseModelBuilder::updated_at_ms)
    pub fn build(self) -> Result<ProjectImageResponseModel, BuildError> {
        Ok(ProjectImageResponseModel {
            image_id: self
                .image_id
                .ok_or_else(|| BuildError::missing_field("image_id"))?,
            filename: self
                .filename
                .ok_or_else(|| BuildError::missing_field("filename"))?,
            signed_url: self.signed_url,
            thumbnail_signed_url: self.thumbnail_signed_url,
            source: self.source,
            file_size_bytes: self
                .file_size_bytes
                .ok_or_else(|| BuildError::missing_field("file_size_bytes"))?,
            width: self
                .width
                .ok_or_else(|| BuildError::missing_field("width"))?,
            height: self
                .height
                .ok_or_else(|| BuildError::missing_field("height"))?,
            track_id: self.track_id,
            offset_ms: self
                .offset_ms
                .ok_or_else(|| BuildError::missing_field("offset_ms"))?,
            duration_ms: self
                .duration_ms
                .ok_or_else(|| BuildError::missing_field("duration_ms"))?,
            order: self
                .order
                .ok_or_else(|| BuildError::missing_field("order"))?,
            canvas_placement: self
                .canvas_placement
                .ok_or_else(|| BuildError::missing_field("canvas_placement"))?,
            animation: self.animation,
            opacity: self.opacity,
            created_at_ms: self
                .created_at_ms
                .ok_or_else(|| BuildError::missing_field("created_at_ms"))?,
            updated_at_ms: self
                .updated_at_ms
                .ok_or_else(|| BuildError::missing_field("updated_at_ms"))?,
            current_snapshot_id: self.current_snapshot_id,
            source_asset_id: self.source_asset_id,
            source_platform_asset_id: self.source_platform_asset_id,
            error: self.error,
            pending_task: self.pending_task,
            analysis: self.analysis,
        })
    }
}
