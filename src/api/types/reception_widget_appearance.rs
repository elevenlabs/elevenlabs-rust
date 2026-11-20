pub use crate::prelude::*;

/// How the website widget looks. A null label follows the receptionist's language.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReceptionWidgetAppearance {
    /// First color of the widget's orb gradient, as a hex color.
    #[serde(default)]
    pub orb_color_1: String,
    /// Second color of the widget's orb gradient, as a hex color.
    #[serde(default)]
    pub orb_color_2: String,
    /// Widget size, from the smallest ('tiny') to the largest ('full').
    pub variant: Variant,
    /// Where on the page the widget sits.
    pub placement: Placement,
    /// Whether visitors can close the widget.
    #[serde(default)]
    pub dismissible: bool,
    /// Text shown on the widget; null uses the default for the receptionist's language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_label: Option<String>,
    /// Text on the button that starts a call; null uses the default for the receptionist's language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_call: Option<String>,
}

impl ReceptionWidgetAppearance {
    pub fn builder() -> ReceptionWidgetAppearanceBuilder {
        <ReceptionWidgetAppearanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionWidgetAppearanceBuilder {
    orb_color_1: Option<String>,
    orb_color_2: Option<String>,
    variant: Option<Variant>,
    placement: Option<Placement>,
    dismissible: Option<bool>,
    main_label: Option<String>,
    start_call: Option<String>,
}

impl ReceptionWidgetAppearanceBuilder {
    pub fn orb_color_1(mut self, value: impl Into<String>) -> Self {
        self.orb_color_1 = Some(value.into());
        self
    }

    pub fn orb_color_2(mut self, value: impl Into<String>) -> Self {
        self.orb_color_2 = Some(value.into());
        self
    }

    pub fn variant(mut self, value: Variant) -> Self {
        self.variant = Some(value);
        self
    }

    pub fn placement(mut self, value: Placement) -> Self {
        self.placement = Some(value);
        self
    }

    pub fn dismissible(mut self, value: bool) -> Self {
        self.dismissible = Some(value);
        self
    }

    pub fn main_label(mut self, value: impl Into<String>) -> Self {
        self.main_label = Some(value.into());
        self
    }

    pub fn start_call(mut self, value: impl Into<String>) -> Self {
        self.start_call = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReceptionWidgetAppearance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`orb_color_1`](ReceptionWidgetAppearanceBuilder::orb_color_1)
    /// - [`orb_color_2`](ReceptionWidgetAppearanceBuilder::orb_color_2)
    /// - [`variant`](ReceptionWidgetAppearanceBuilder::variant)
    /// - [`placement`](ReceptionWidgetAppearanceBuilder::placement)
    /// - [`dismissible`](ReceptionWidgetAppearanceBuilder::dismissible)
    pub fn build(self) -> Result<ReceptionWidgetAppearance, BuildError> {
        Ok(ReceptionWidgetAppearance {
            orb_color_1: self
                .orb_color_1
                .ok_or_else(|| BuildError::missing_field("orb_color_1"))?,
            orb_color_2: self
                .orb_color_2
                .ok_or_else(|| BuildError::missing_field("orb_color_2"))?,
            variant: self
                .variant
                .ok_or_else(|| BuildError::missing_field("variant"))?,
            placement: self
                .placement
                .ok_or_else(|| BuildError::missing_field("placement"))?,
            dismissible: self
                .dismissible
                .ok_or_else(|| BuildError::missing_field("dismissible"))?,
            main_label: self.main_label,
            start_call: self.start_call,
        })
    }
}
