pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum QueryParamsJsonSchemaInputProperties {
    LiteralJsonSchemaProperty(LiteralJsonSchemaProperty),
    QueryArrayJsonSchemaPropertyInput(QueryArrayJsonSchemaPropertyInput),
}

impl<'de> serde::Deserialize<'de> for QueryParamsJsonSchemaInputProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let matching_keys = |keys: &[&str]| {
            value
                .as_object()
                .map(|object| {
                    object
                        .keys()
                        .filter(|key| keys.contains(&key.as_str()))
                        .count()
                })
                .unwrap_or(0)
        };

        let mut matched_literal = false;

        if value.get("type").and_then(serde_json::Value::as_str) == Some("array") {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<QueryArrayJsonSchemaPropertyInput>(value.clone())
            {
                let best = matching_keys(&[
                    "property_kind",
                    "description",
                    "dynamic_variable",
                    "constant_value",
                    "is_omitted",
                    "type",
                    "items",
                ]);
                let mut chosen = None;
                let score = matching_keys(&[
                    "type",
                    "description",
                    "enum",
                    "is_system_provided",
                    "dynamic_variable",
                    "allowed_values",
                    "allowed_values_dynamic_variable",
                    "constant_value",
                    "is_omitted",
                ]);
                if score > best {
                    if let Ok(alternative) =
                        serde_json::from_value::<LiteralJsonSchemaProperty>(value.clone())
                    {
                        chosen = Some(Self::LiteralJsonSchemaProperty(alternative));
                    }
                }
                return Ok(chosen.unwrap_or(Self::QueryArrayJsonSchemaPropertyInput(parsed)));
            }
        }

        if !matched_literal {
            if let Ok(parsed) = serde_json::from_value::<LiteralJsonSchemaProperty>(value.clone()) {
                return Ok(Self::LiteralJsonSchemaProperty(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<QueryArrayJsonSchemaPropertyInput>(value.clone())
            {
                return Ok(Self::QueryArrayJsonSchemaPropertyInput(parsed));
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum QueryParamsJsonSchemaInputProperties",
        ))
    }
}

impl QueryParamsJsonSchemaInputProperties {
    pub fn is_literal_json_schema_property(&self) -> bool {
        matches!(self, Self::LiteralJsonSchemaProperty(_))
    }

    pub fn as_literal_json_schema_property(&self) -> Option<&LiteralJsonSchemaProperty> {
        match self {
            Self::LiteralJsonSchemaProperty(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_literal_json_schema_property(self) -> Option<LiteralJsonSchemaProperty> {
        match self {
            Self::LiteralJsonSchemaProperty(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_query_array_json_schema_property_input(&self) -> bool {
        matches!(self, Self::QueryArrayJsonSchemaPropertyInput(_))
    }

    pub fn as_query_array_json_schema_property_input(
        &self,
    ) -> Option<&QueryArrayJsonSchemaPropertyInput> {
        match self {
            Self::QueryArrayJsonSchemaPropertyInput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_query_array_json_schema_property_input(
        self,
    ) -> Option<QueryArrayJsonSchemaPropertyInput> {
        match self {
            Self::QueryArrayJsonSchemaPropertyInput(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for QueryParamsJsonSchemaInputProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LiteralJsonSchemaProperty(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::QueryArrayJsonSchemaPropertyInput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
