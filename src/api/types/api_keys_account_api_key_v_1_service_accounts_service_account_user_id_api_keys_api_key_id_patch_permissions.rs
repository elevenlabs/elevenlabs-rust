pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum AccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatchPermissions {
    PermissionTypeList(Vec<PermissionType>),
    Permissions2_2(Permissions2_2),
}

impl AccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatchPermissions {
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

    pub fn is_permissions2_2(&self) -> bool {
        matches!(self, Self::Permissions2_2(_))
    }

    pub fn as_permissions2_2(&self) -> Option<&Permissions2_2> {
        match self {
            Self::Permissions2_2(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_permissions2_2(self) -> Option<Permissions2_2> {
        match self {
            Self::Permissions2_2(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display
    for AccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatchPermissions
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PermissionTypeList(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::Permissions2_2(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for AccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatchPermissions {
    fn default() -> Self {
        AccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatchPermissions::PermissionTypeList(Default::default())
    }
}
