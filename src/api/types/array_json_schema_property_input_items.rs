pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ArrayJsonSchemaPropertyInputItems {
    LiteralJsonSchemaProperty(LiteralJsonSchemaProperty),
    ObjectJsonSchemaPropertyInput(ObjectJsonSchemaPropertyInput),
    ArrayJsonSchemaPropertyInput(ArrayJsonSchemaPropertyInput),
}

impl<'de> serde::Deserialize<'de> for ArrayJsonSchemaPropertyInputItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let has_any_key = |keys: &[&str]| {
            value
                .as_object()
                .map(|object| keys.iter().any(|key| object.contains_key(*key)))
                .unwrap_or(false)
        };
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

        if value.get("type").and_then(serde_json::Value::as_str) == Some("object") {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<ObjectJsonSchemaPropertyInput>(value.clone())
            {
                let best = matching_keys(&[
                    "property_kind",
                    "description",
                    "dynamic_variable",
                    "constant_value",
                    "is_omitted",
                    "type",
                    "required",
                    "properties",
                    "required_constraints",
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
                return Ok(chosen.unwrap_or(Self::ObjectJsonSchemaPropertyInput(parsed)));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("array") {
            matched_literal = true;
            if let Ok(parsed) =
                serde_json::from_value::<ArrayJsonSchemaPropertyInput>(value.clone())
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
                return Ok(chosen.unwrap_or(Self::ArrayJsonSchemaPropertyInput(parsed)));
            }
        }

        if !matched_literal {
            if let Ok(parsed) = serde_json::from_value::<LiteralJsonSchemaProperty>(value.clone()) {
                return Ok(Self::LiteralJsonSchemaProperty(parsed));
            }

            if has_any_key(&[
                "property_kind",
                "description",
                "dynamic_variable",
                "constant_value",
                "is_omitted",
                "type",
                "required",
                "properties",
                "required_constraints",
            ]) {
                if let Ok(parsed) =
                    serde_json::from_value::<ObjectJsonSchemaPropertyInput>(value.clone())
                {
                    return Ok(Self::ObjectJsonSchemaPropertyInput(parsed));
                }
            }

            if has_any_key(&[
                "property_kind",
                "description",
                "dynamic_variable",
                "constant_value",
                "is_omitted",
                "type",
                "items",
            ]) {
                if let Ok(parsed) =
                    serde_json::from_value::<ArrayJsonSchemaPropertyInput>(value.clone())
                {
                    return Ok(Self::ArrayJsonSchemaPropertyInput(parsed));
                }
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum ArrayJsonSchemaPropertyInputItems",
        ))
    }
}

impl ArrayJsonSchemaPropertyInputItems {
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

    pub fn is_object_json_schema_property_input(&self) -> bool {
        matches!(self, Self::ObjectJsonSchemaPropertyInput(_))
    }

    pub fn as_object_json_schema_property_input(&self) -> Option<&ObjectJsonSchemaPropertyInput> {
        match self {
            Self::ObjectJsonSchemaPropertyInput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_object_json_schema_property_input(self) -> Option<ObjectJsonSchemaPropertyInput> {
        match self {
            Self::ObjectJsonSchemaPropertyInput(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_array_json_schema_property_input(&self) -> bool {
        matches!(self, Self::ArrayJsonSchemaPropertyInput(_))
    }

    pub fn as_array_json_schema_property_input(&self) -> Option<&ArrayJsonSchemaPropertyInput> {
        match self {
            Self::ArrayJsonSchemaPropertyInput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_array_json_schema_property_input(self) -> Option<ArrayJsonSchemaPropertyInput> {
        match self {
            Self::ArrayJsonSchemaPropertyInput(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ArrayJsonSchemaPropertyInputItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LiteralJsonSchemaProperty(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ObjectJsonSchemaPropertyInput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ArrayJsonSchemaPropertyInput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for ArrayJsonSchemaPropertyInputItems {
    fn default() -> Self {
        ArrayJsonSchemaPropertyInputItems::ObjectJsonSchemaPropertyInput(Default::default())
    }
}
