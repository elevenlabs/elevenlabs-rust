pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<UserActivityType>,
}

impl UserActivity {
    pub fn builder() -> UserActivityBuilder {
        <UserActivityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserActivityBuilder {
    r#type: Option<UserActivityType>,
}

impl UserActivityBuilder {
    pub fn r#type(mut self, value: UserActivityType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserActivity`].
    pub fn build(self) -> Result<UserActivity, BuildError> {
        Ok(UserActivity {
            r#type: self.r#type,
        })
    }
}
