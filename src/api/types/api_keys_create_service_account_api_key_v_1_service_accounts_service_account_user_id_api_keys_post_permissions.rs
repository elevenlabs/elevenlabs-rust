pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPostPermissions {
    PermissionTypeList(Vec<PermissionType>),
    Permissions2_1(Permissions2_1),
}

impl CreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPostPermissions {
    pub fn is_permission_type_list(&self) -> bool {
        matches!(self, Self::PermissionTypeList(_))
    }

    pub fn as_permission_type_list(&self) -> Option<&Vec<PermissionType>> {
        match self {
            Self::PermissionTypeList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_permission_type_list(self) -> Option<Vec<PermissionType>> {
        match self {
            Self::PermissionTypeList(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_permissions2_1(&self) -> bool {
        matches!(self, Self::Permissions2_1(_))
    }

    pub fn as_permissions2_1(&self) -> Option<&Permissions2_1> {
        match self {
            Self::Permissions2_1(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_permissions2_1(self) -> Option<Permissions2_1> {
        match self {
            Self::Permissions2_1(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display
    for CreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPostPermissions
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PermissionTypeList(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::Permissions2_1(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default
    for CreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPostPermissions
{
    fn default() -> Self {
        CreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPostPermissions::PermissionTypeList(Default::default())
    }
}
