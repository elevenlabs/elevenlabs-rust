pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum QueryParamsJsonSchemaOutputProperties {
    LiteralJsonSchemaProperty(LiteralJsonSchemaProperty),
    QueryArrayJsonSchemaPropertyOutput(QueryArrayJsonSchemaPropertyOutput),
}

impl<'de> serde::Deserialize<'de> for QueryParamsJsonSchemaOutputProperties {
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
                serde_json::from_value::<QueryArrayJsonSchemaPropertyOutput>(value.clone())
            {
                let best = matching_keys(&[
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
                return Ok(chosen.unwrap_or(Self::QueryArrayJsonSchemaPropertyOutput(parsed)));
            }
        }

        if !matched_literal {
            if let Ok(parsed) = serde_json::from_value::<LiteralJsonSchemaProperty>(value.clone()) {
                return Ok(Self::LiteralJsonSchemaProperty(parsed));
            }

            if let Ok(parsed) =
                serde_json::from_value::<QueryArrayJsonSchemaPropertyOutput>(value.clone())
            {
                return Ok(Self::QueryArrayJsonSchemaPropertyOutput(parsed));
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum QueryParamsJsonSchemaOutputProperties",
        ))
    }
}

impl QueryParamsJsonSchemaOutputProperties {
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

    pub fn is_query_array_json_schema_property_output(&self) -> bool {
        matches!(self, Self::QueryArrayJsonSchemaPropertyOutput(_))
    }

    pub fn as_query_array_json_schema_property_output(
        &self,
    ) -> Option<&QueryArrayJsonSchemaPropertyOutput> {
        match self {
            Self::QueryArrayJsonSchemaPropertyOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_query_array_json_schema_property_output(
        self,
    ) -> Option<QueryArrayJsonSchemaPropertyOutput> {
        match self {
            Self::QueryArrayJsonSchemaPropertyOutput(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for QueryParamsJsonSchemaOutputProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LiteralJsonSchemaProperty(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::QueryArrayJsonSchemaPropertyOutput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
