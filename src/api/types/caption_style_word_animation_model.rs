pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CaptionStyleWordAnimationModel {
    pub enter_type: CaptionStyleSectionAnimationModelEnterType,
    pub exit_type: CaptionStyleSectionAnimationModelEnterType,
}

impl CaptionStyleWordAnimationModel {
    pub fn builder() -> CaptionStyleWordAnimationModelBuilder {
        <CaptionStyleWordAnimationModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptionStyleWordAnimationModelBuilder {
    enter_type: Option<CaptionStyleSectionAnimationModelEnterType>,
    exit_type: Option<CaptionStyleSectionAnimationModelEnterType>,
}

impl CaptionStyleWordAnimationModelBuilder {
    pub fn enter_type(mut self, value: CaptionStyleSectionAnimationModelEnterType) -> Self {
        self.enter_type = Some(value);
        self
    }

    pub fn exit_type(mut self, value: CaptionStyleSectionAnimationModelEnterType) -> Self {
        self.exit_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptionStyleWordAnimationModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enter_type`](CaptionStyleWordAnimationModelBuilder::enter_type)
    /// - [`exit_type`](CaptionStyleWordAnimationModelBuilder::exit_type)
    pub fn build(self) -> Result<CaptionStyleWordAnimationModel, BuildError> {
        Ok(CaptionStyleWordAnimationModel {
            enter_type: self
                .enter_type
                .ok_or_else(|| BuildError::missing_field("enter_type"))?,
            exit_type: self
                .exit_type
                .ok_or_else(|| BuildError::missing_field("exit_type"))?,
        })
    }
}
