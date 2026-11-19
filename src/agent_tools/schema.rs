//! `Schema`: the JSON-schema dialect the API uses for tool params, and where each value comes from.
//!
//! Schemas are values: every method takes the schema by value and returns the changed copy, so
//! clone one to reuse it across tools and properties.

use serde_json::{json, Map, Value};

use crate::agent_workflows::wire::canonical;

pub(crate) type WireMap = Map<String, Value>;

/// The kind of a schema: one of the four literals, an object or an array.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SchemaKind {
    String,
    Integer,
    Number,
    Boolean,
    Object,
    Array,
}

impl SchemaKind {
    /// The wire `type` name: `"string"`, `"integer"`, `"number"`, `"boolean"`, `"object"` or
    /// `"array"`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Integer => "integer",
            Self::Number => "number",
            Self::Boolean => "boolean",
            Self::Object => "object",
            Self::Array => "array",
        }
    }

    /// Whether this is a string, integer, number or boolean.
    pub fn is_literal(self) -> bool {
        !matches!(self, Self::Object | Self::Array)
    }
}

/// Everything a schema holds. Modeled fields are typed; loaded fields the builder doesn't model
/// (and loaded API defaults for unset fields) stay in `extra` and are written back unchanged.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SchemaState {
    pub kind: SchemaKind,
    /// A loaded literal's `type`, kept as is (the API also allows a list such as
    /// `["integer", "null"]`).
    pub literal_type: Option<Value>,
    /// Whether the loaded schema declared a `type` (otherwise the builder writes one).
    pub type_loaded: bool,
    pub description: Option<String>,
    pub dynamic_variable: Option<String>,
    /// `Some` when a constant is set; the value itself may be `null`.
    pub constant: Option<Value>,
    pub system_provided: bool,
    pub omitted: bool,
    pub optional: bool,
    pub choices: Option<Value>,
    pub allowed_from: Option<String>,
    pub items: Option<Box<Schema>>,
    pub properties: Vec<(String, Schema)>,
    /// A loaded object's `required`, so its order survives edits.
    pub loaded_required: Option<Vec<String>>,
    pub any_of: Vec<Vec<String>>,
    pub all_of: Vec<Vec<String>>,
    pub extra: WireMap,
    /// Loaded modeled fields of the wrong shape, kept in `extra` as loaded. The generated request
    /// model would drop some of them silently, so `build()` reports each one.
    pub malformed: Vec<&'static str>,
}

impl SchemaState {
    fn initial(kind: SchemaKind) -> Self {
        Self {
            kind,
            literal_type: None,
            type_loaded: false,
            description: None,
            dynamic_variable: None,
            constant: None,
            system_provided: false,
            omitted: false,
            optional: false,
            choices: None,
            allowed_from: None,
            items: None,
            properties: Vec::new(),
            loaded_required: None,
            any_of: Vec::new(),
            all_of: Vec::new(),
            extra: WireMap::new(),
            malformed: Vec::new(),
        }
    }

    /// The schema in wire format (unsorted). Modeled fields that are unset leave the matching
    /// extra field (an API default) in place.
    fn to_model(&self) -> WireMap {
        let mut out = self.extra.clone();
        if let Some(description) = &self.description {
            out.insert("description".into(), json!(description));
        }
        if let Some(variable) = &self.dynamic_variable {
            out.insert("dynamic_variable".into(), json!(variable));
        }
        if self.omitted {
            out.insert("is_omitted".into(), json!(true));
        }
        if let Some(constant) = &self.constant {
            out.insert("constant_value".into(), constant.clone());
        }
        match self.kind {
            SchemaKind::Object => {
                out.insert("type".into(), json!("object"));
                if !self.properties.is_empty() {
                    let properties: WireMap = self
                        .properties
                        .iter()
                        .map(|(name, child)| (name.clone(), Value::Object(child.to_model())))
                        .collect();
                    out.insert("properties".into(), Value::Object(properties));
                }
                let required = self.required();
                if !required.is_empty() {
                    out.insert("required".into(), json!(required));
                }
                let mut constraints = WireMap::new();
                for (key, groups) in [("any_of", &self.any_of), ("all_of", &self.all_of)] {
                    if !groups.is_empty() {
                        let entries: Vec<Value> = groups
                            .iter()
                            .map(|names| json!({ "required": names }))
                            .collect();
                        constraints.insert(key.into(), Value::Array(entries));
                    }
                }
                if !constraints.is_empty() {
                    out.insert("required_constraints".into(), Value::Object(constraints));
                }
            }
            SchemaKind::Array => {
                out.insert("type".into(), json!("array"));
                if let Some(items) = &self.items {
                    out.insert("items".into(), Value::Object(items.to_model()));
                }
            }
            kind => {
                let literal_type = self
                    .literal_type
                    .clone()
                    .unwrap_or_else(|| json!(kind.as_str()));
                out.insert("type".into(), literal_type);
                if self.system_provided {
                    out.insert("is_system_provided".into(), json!(true));
                }
                if let Some(choices) = &self.choices {
                    out.insert("enum".into(), choices.clone());
                }
                if let Some(variable) = &self.allowed_from {
                    out.insert(
                        "allowed_values".into(),
                        json!({ "dynamic_variable": variable }),
                    );
                }
            }
        }
        out
    }

    /// A loaded object's `required` order for the required properties it still has, then new
    /// ones in property order.
    fn required(&self) -> Vec<String> {
        let required: Vec<&String> = self
            .properties
            .iter()
            .filter(|(_, child)| !child.is_optional())
            .map(|(name, _)| name)
            .collect();
        let mut out: Vec<String> = self
            .loaded_required
            .iter()
            .flatten()
            .filter(|name| required.contains(name))
            .cloned()
            .collect();
        for name in required {
            if !out.contains(name) {
                out.push(name.clone());
            }
        }
        out
    }
}

/// The methods every schema kind has. Each takes the schema by value and returns the same type.
macro_rules! common_schema_methods {
    () => {
        /// `String`, `Integer`, `Number`, `Boolean`, `Object` or `Array`.
        pub fn kind(&self) -> SchemaKind {
            self.state.kind
        }

        /// Whether the parent leaves this property out of `required`.
        pub fn is_optional(&self) -> bool {
            self.state.optional
        }

        /// The LLM provides the value, guided by `text`. On an object or array, documents the
        /// container instead.
        pub fn describe(mut self, text: impl Into<String>) -> Self {
            self.state.description = Some(text.into());
            self
        }

        /// The value is read from the dynamic variable `name` when the tool runs.
        pub fn from_variable(mut self, name: impl Into<String>) -> Self {
            self.state.dynamic_variable = Some(name.into());
            self
        }

        /// The value is always `value`, a JSON value matching the schema's type (checked by
        /// `build()`).
        pub fn constant(mut self, value: impl Into<Value>) -> Self {
            self.state.constant = Some(value.into());
            self
        }

        /// The param is left out of the request. Only allowed on an optional property.
        pub fn omit(mut self) -> Self {
            self.state.omitted = true;
            self
        }

        /// Leaves this property out of its parent's `required`. Properties are required unless
        /// marked optional.
        pub fn optional(mut self) -> Self {
            self.state.optional = true;
            self
        }

        /// The schema in API format (snake_case keys, sorted). Doesn't validate.
        pub fn to_json(&self) -> Value {
            canonical(&Value::Object(self.state.to_model()))
        }
    };
}

/// A string, integer, number or boolean. Create one with [`Schema::string`],
/// [`Schema::integer`], [`Schema::number`] or [`Schema::bool`].
#[derive(Debug, Clone, PartialEq)]
pub struct LiteralSchema {
    state: Box<SchemaState>,
}

impl LiteralSchema {
    common_schema_methods!();

    /// The values the LLM may choose from (`enum`). Strings only. Named `choices` because `enum`
    /// is reserved.
    pub fn choices<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let values: Vec<Value> = values
            .into_iter()
            .map(|v| Value::String(v.into()))
            .collect();
        self.state.choices = Some(Value::Array(values));
        self
    }

    /// The value is filled in by the system at runtime.
    pub fn system_provided(mut self) -> Self {
        self.state.system_provided = true;
        self
    }

    /// Rejects an LLM value that isn't in the list held by the dynamic variable `variable`. The
    /// list isn't shown to the LLM. Needs `describe()`.
    pub fn allowed_from(mut self, variable: impl Into<String>) -> Self {
        self.state.allowed_from = Some(variable.into());
        self
    }
}

/// An object with named properties. Create one with [`Schema::object`] and add properties with
/// [`property`](Self::property).
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectSchema {
    state: Box<SchemaState>,
}

impl ObjectSchema {
    common_schema_methods!();

    /// Adds the property `name`, or replaces it in place if it exists.
    pub fn property(mut self, name: impl Into<String>, schema: impl Into<Schema>) -> Self {
        let name = name.into();
        let schema = schema.into();
        match self
            .state
            .properties
            .iter_mut()
            .find(|(existing, _)| *existing == name)
        {
            Some(slot) => slot.1 = schema,
            None => self.state.properties.push((name, schema)),
        }
        self
    }

    /// Removes the property `name`.
    pub fn without(mut self, name: &str) -> Self {
        self.state
            .properties
            .retain(|(existing, _)| existing != name);
        self
    }

    /// Requires at least one of these properties: appends `names` to
    /// `required_constraints.any_of`.
    pub fn require_any_of<I, S>(mut self, names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.state
            .any_of
            .push(names.into_iter().map(Into::into).collect());
        self
    }

    /// Requires all of these properties: appends `names` to `required_constraints.all_of`.
    pub fn require_all_of<I, S>(mut self, names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.state
            .all_of
            .push(names.into_iter().map(Into::into).collect());
        self
    }

    /// Property names, in order.
    pub fn property_names(&self) -> Vec<&str> {
        self.state
            .properties
            .iter()
            .map(|(name, _)| name.as_str())
            .collect()
    }

    /// The schema of property `name`, if there is one.
    pub fn property_schema(&self, name: &str) -> Option<&Schema> {
        self.state
            .properties
            .iter()
            .find(|(existing, _)| existing == name)
            .map(|(_, schema)| schema)
    }
}

/// A list whose elements match one schema. Create one with [`Schema::array`].
#[derive(Debug, Clone, PartialEq)]
pub struct ArraySchema {
    state: Box<SchemaState>,
}

impl ArraySchema {
    common_schema_methods!();

    /// The element schema. Always set unless the schema was loaded without one.
    pub fn items(&self) -> Option<&Schema> {
        self.state.items.as_deref()
    }
}

/// A tool param's JSON schema, and where its value comes from.
///
/// The constructors return the kind-specific type ([`LiteralSchema`], [`ObjectSchema`],
/// [`ArraySchema`]), so a method that only applies to one kind doesn't compile on another. Every
/// method that takes a schema accepts any of them (`impl Into<Schema>`).
///
/// ```
/// use elevenlabs_rust_sdk::agent_tools::Schema;
///
/// let body = Schema::object()
///     .property("order_id", Schema::string().from_variable("order_id"))
///     .property(
///         "reason",
///         Schema::string()
///             .describe("Why the caller wants a refund")
///             .choices(["damaged", "late"]),
///     )
///     .property("note", Schema::string().describe("Anything else the caller said").optional());
/// assert_eq!(body.property_names(), ["order_id", "reason", "note"]);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Schema {
    Literal(LiteralSchema),
    Object(ObjectSchema),
    Array(ArraySchema),
}

impl From<LiteralSchema> for Schema {
    fn from(schema: LiteralSchema) -> Self {
        Schema::Literal(schema)
    }
}

impl From<ObjectSchema> for Schema {
    fn from(schema: ObjectSchema) -> Self {
        Schema::Object(schema)
    }
}

impl From<ArraySchema> for Schema {
    fn from(schema: ArraySchema) -> Self {
        Schema::Array(schema)
    }
}

impl Schema {
    /// A string.
    pub fn string() -> LiteralSchema {
        LiteralSchema {
            state: Box::new(SchemaState::initial(SchemaKind::String)),
        }
    }

    /// An integer.
    pub fn integer() -> LiteralSchema {
        LiteralSchema {
            state: Box::new(SchemaState::initial(SchemaKind::Integer)),
        }
    }

    /// A number.
    pub fn number() -> LiteralSchema {
        LiteralSchema {
            state: Box::new(SchemaState::initial(SchemaKind::Number)),
        }
    }

    /// A boolean. Named `bool` (not `boolean`, which is reserved in Java) to match every SDK.
    pub fn bool() -> LiteralSchema {
        LiteralSchema {
            state: Box::new(SchemaState::initial(SchemaKind::Boolean)),
        }
    }

    /// An object. Add properties with [`ObjectSchema::property`].
    pub fn object() -> ObjectSchema {
        ObjectSchema {
            state: Box::new(SchemaState::initial(SchemaKind::Object)),
        }
    }

    /// An array whose elements match `items`.
    pub fn array(items: impl Into<Schema>) -> ArraySchema {
        let mut state = SchemaState::initial(SchemaKind::Array);
        state.items = Some(Box::new(items.into()));
        ArraySchema {
            state: Box::new(state),
        }
    }

    /// Loads a schema in API format (snake_case). A value that isn't an object loads as an empty
    /// string schema.
    pub fn from_json(json: &Value) -> Schema {
        let model = strip_nulls(json.clone())
            .as_object()
            .cloned()
            .unwrap_or_default();
        let kind = kind_of(&model);
        Schema::from_model(&model, false, kind)
    }

    /// Loads a schema from an API-format JSON string.
    pub fn from_json_str(json: &str) -> Result<Schema, serde_json::Error> {
        Ok(Schema::from_json(&serde_json::from_str(json)?))
    }

    /// `String`, `Integer`, `Number`, `Boolean`, `Object` or `Array`.
    pub fn kind(&self) -> SchemaKind {
        self.state().kind
    }

    /// Whether the parent leaves this property out of `required`.
    pub fn is_optional(&self) -> bool {
        self.state().optional
    }

    /// The LLM provides the value, guided by `text`.
    pub fn describe(self, text: impl Into<String>) -> Self {
        self.with_state(|s| s.description = Some(text.into()))
    }

    /// The value is read from the dynamic variable `name` when the tool runs.
    pub fn from_variable(self, name: impl Into<String>) -> Self {
        self.with_state(|s| s.dynamic_variable = Some(name.into()))
    }

    /// The value is always `value`.
    pub fn constant(self, value: impl Into<Value>) -> Self {
        self.with_state(|s| s.constant = Some(value.into()))
    }

    /// The param is left out of the request. Only allowed on an optional property.
    pub fn omit(self) -> Self {
        self.with_state(|s| s.omitted = true)
    }

    /// Leaves this property out of its parent's `required`.
    pub fn optional(self) -> Self {
        self.with_state(|s| s.optional = true)
    }

    /// The schema in API format (snake_case keys, sorted). Doesn't validate.
    pub fn to_json(&self) -> Value {
        canonical(&Value::Object(self.to_model()))
    }

    /// This schema as a literal, or `None` if it's another kind.
    pub fn as_literal(&self) -> Option<&LiteralSchema> {
        match self {
            Schema::Literal(s) => Some(s),
            _ => None,
        }
    }

    /// This schema as an object, or `None` if it's another kind.
    pub fn as_object(&self) -> Option<&ObjectSchema> {
        match self {
            Schema::Object(s) => Some(s),
            _ => None,
        }
    }

    /// This schema as an array, or `None` if it's another kind.
    pub fn as_array(&self) -> Option<&ArraySchema> {
        match self {
            Schema::Array(s) => Some(s),
            _ => None,
        }
    }

    /// This schema as a literal, or `None` if it's another kind.
    pub fn into_literal(self) -> Option<LiteralSchema> {
        match self {
            Schema::Literal(s) => Some(s),
            _ => None,
        }
    }

    /// This schema as an object, or `None` if it's another kind.
    pub fn into_object(self) -> Option<ObjectSchema> {
        match self {
            Schema::Object(s) => Some(s),
            _ => None,
        }
    }

    /// This schema as an array, or `None` if it's another kind.
    pub fn into_array(self) -> Option<ArraySchema> {
        match self {
            Schema::Array(s) => Some(s),
            _ => None,
        }
    }

    pub(crate) fn state(&self) -> &SchemaState {
        match self {
            Schema::Literal(s) => &s.state,
            Schema::Object(s) => &s.state,
            Schema::Array(s) => &s.state,
        }
    }

    fn with_state(mut self, change: impl FnOnce(&mut SchemaState)) -> Self {
        let state = match &mut self {
            Schema::Literal(s) => &mut *s.state,
            Schema::Object(s) => &mut *s.state,
            Schema::Array(s) => &mut *s.state,
        };
        change(state);
        self
    }

    /// The schema in wire format, unsorted.
    pub(crate) fn to_model(&self) -> WireMap {
        self.state().to_model()
    }

    /// Loads a wire-format schema. `optional` is whether its parent leaves it out of `required`.
    /// `kind` is given where the API only allows one kind (e.g. a tool's params object), since an
    /// empty schema has no `type`.
    pub(crate) fn from_model(model: &WireMap, optional: bool, kind: SchemaKind) -> Schema {
        // The API fills unset fields with defaults ("", false, []). They aren't value sources, but
        // are kept as extra fields so a pulled config writes back unchanged. `null` is absent.
        let mut extra = WireMap::new();
        for (key, value) in model {
            if !value.is_null() && (!is_modeled(kind, key) || is_unset(key, value, kind)) {
                extra.insert(key.clone(), value.clone());
            }
        }
        let set = |key: &str| {
            model
                .get(key)
                .filter(|value| !value.is_null() && !is_unset(key, value, kind))
        };
        let mut state = SchemaState::initial(kind);
        state.optional = optional;
        for key in ["description", "dynamic_variable"] {
            match set(key) {
                Some(Value::String(text)) => {
                    if key == "description" {
                        state.description = Some(text.clone());
                    } else {
                        state.dynamic_variable = Some(text.clone());
                    }
                }
                Some(other) => {
                    extra.insert(key.into(), other.clone());
                }
                None => {}
            }
        }
        // `null` is absent, except as the constant of a literal whose type list includes "null".
        let null_constant = kind.is_literal()
            && model.get("constant_value") == Some(&Value::Null)
            && is_nullable(model.get("type"));
        state.constant = if null_constant {
            Some(Value::Null)
        } else {
            set("constant_value").cloned()
        };
        state.type_loaded = model.get("type").is_some_and(|t| !t.is_null());
        // A modeled field whose value has the wrong shape isn't read: it stays in `extra`, so it
        // is written back as loaded and `build()` reports it (`invalid_model`) instead of the
        // builder dropping it.
        let mut malformed = Vec::new();
        let mut keep = |key: &'static str, wrong_shape: bool| {
            if let Some(value) = set(key) {
                extra.insert(key.into(), value.clone());
                if wrong_shape {
                    malformed.push(key);
                }
            }
        };
        match set("is_omitted") {
            Some(Value::Bool(true)) => state.omitted = true,
            Some(_) => keep("is_omitted", true),
            None => {}
        }
        match kind {
            SchemaKind::Object => {
                let properties = set("properties").map(|properties| {
                    properties
                        .as_object()
                        .filter(|properties| properties.values().all(Value::is_object))
                });
                let required =
                    set("required").map(|names| names.as_array().and_then(|n| names_of(n)));
                let (bad_properties, bad_required) =
                    (properties == Some(None), required == Some(None));
                match (properties, required) {
                    // Each needs the other: a property's optional() comes from `required`.
                    _ if bad_properties || bad_required => {
                        keep("properties", bad_properties);
                        keep("required", bad_required);
                    }
                    (properties, required) => {
                        let required = required.flatten();
                        let required_names = required.clone().unwrap_or_default();
                        for (name, child) in properties.flatten().into_iter().flatten() {
                            let child = child.as_object().cloned().unwrap_or_default();
                            let optional = !required_names.contains(name);
                            let schema = Schema::from_model(&child, optional, kind_of(&child));
                            state.properties.push((name.clone(), schema));
                        }
                        state.loaded_required = required;
                    }
                }
                if let Some(constraints) = set("required_constraints") {
                    match required_constraints(constraints) {
                        Some((any_of, all_of)) => {
                            state.any_of = any_of;
                            state.all_of = all_of;
                        }
                        None => keep("required_constraints", true),
                    }
                }
                state.extra = extra;
                state.malformed = malformed;
                Schema::Object(ObjectSchema {
                    state: Box::new(state),
                })
            }
            SchemaKind::Array => {
                match set("items") {
                    Some(Value::Object(items)) => {
                        let kind = kind_of(items);
                        state.items = Some(Box::new(Schema::from_model(items, true, kind)));
                    }
                    Some(_) => keep("items", true),
                    None => {}
                }
                state.extra = extra;
                state.malformed = malformed;
                Schema::Array(ArraySchema {
                    state: Box::new(state),
                })
            }
            _ => {
                state.literal_type = model.get("type").filter(|t| !t.is_null()).cloned();
                match set("is_system_provided") {
                    Some(Value::Bool(true)) => state.system_provided = true,
                    Some(_) => keep("is_system_provided", true),
                    None => {}
                }
                state.choices = set("enum").cloned();
                match set("allowed_values") {
                    Some(Value::Object(allowed)) if allowed.len() == 1 => {
                        match allowed.get("dynamic_variable") {
                            Some(Value::String(variable)) => {
                                state.allowed_from = Some(variable.clone());
                            }
                            _ => keep("allowed_values", true),
                        }
                    }
                    Some(_) => keep("allowed_values", true),
                    None => {}
                }
                state.extra = extra;
                state.malformed = malformed;
                Schema::Literal(LiteralSchema {
                    state: Box::new(state),
                })
            }
        }
    }
}

/// Whether the builder models `key` for a schema of `kind`. Anything else is kept as loaded.
fn is_modeled(kind: SchemaKind, key: &str) -> bool {
    if matches!(
        key,
        "type" | "description" | "dynamic_variable" | "constant_value" | "is_omitted"
    ) {
        return true;
    }
    match kind {
        SchemaKind::Object => matches!(key, "properties" | "required" | "required_constraints"),
        SchemaKind::Array => key == "items",
        _ => matches!(key, "is_system_provided" | "enum" | "allowed_values"),
    }
}

/// Whether `value` is what the API returns for an unset field. A literal's unset constant is
/// `""`, so `""` can't be a literal constant.
fn is_unset(key: &str, value: &Value, kind: SchemaKind) -> bool {
    match key {
        "description" | "dynamic_variable" => value.as_str() == Some(""),
        "constant_value" => kind.is_literal() && value.as_str() == Some(""),
        "is_system_provided" | "is_omitted" => value == &Value::Bool(false),
        "required" => value.as_array().is_some_and(|a| a.is_empty()),
        "properties" => value.as_object().is_some_and(|o| o.is_empty()),
        _ => false,
    }
}

/// The kind of a wire-format schema. A literal's type may be a list such as
/// `["integer", "null"]`; its kind is the non-null member.
pub(crate) fn kind_of(model: &WireMap) -> SchemaKind {
    let ty = model.get("type").filter(|t| !t.is_null());
    if ty.and_then(Value::as_str) == Some("object")
        || (ty.is_none() && model.contains_key("properties"))
    {
        return SchemaKind::Object;
    }
    if ty.and_then(Value::as_str) == Some("array") || (ty.is_none() && model.contains_key("items"))
    {
        return SchemaKind::Array;
    }
    let name = match ty {
        Some(Value::Array(list)) => list.iter().filter_map(Value::as_str).find(|t| *t != "null"),
        Some(Value::String(t)) => Some(t.as_str()),
        _ => None,
    };
    match name {
        Some("integer") => SchemaKind::Integer,
        Some("number") => SchemaKind::Number,
        Some("boolean") => SchemaKind::Boolean,
        _ => SchemaKind::String,
    }
}

/// `values` as strings, or `None` if one isn't a string.
fn names_of(values: &[Value]) -> Option<Vec<String>> {
    values
        .iter()
        .map(|value| value.as_str().map(str::to_string))
        .collect()
}

/// Groups of property names, as in `required_constraints.any_of`.
type Groups = Vec<Vec<String>>;

/// A loaded `required_constraints`' `any_of` and `all_of` groups, or `None` if it has another
/// shape (fields other than these two, or a group that isn't `{"required": [names]}`).
fn required_constraints(value: &Value) -> Option<(Groups, Groups)> {
    let constraints = value.as_object()?;
    if constraints
        .keys()
        .any(|key| key != "any_of" && key != "all_of")
    {
        return None;
    }
    let groups = |key: &str| -> Option<Groups> {
        match constraints.get(key) {
            None | Some(Value::Null) => Some(Vec::new()),
            Some(Value::Array(groups)) => groups
                .iter()
                .map(|group| {
                    let group = group.as_object().filter(|group| group.len() == 1)?;
                    names_of(group.get("required")?.as_array()?)
                })
                .collect(),
            Some(_) => None,
        }
    };
    Some((groups("any_of")?, groups("all_of")?))
}

/// Whether a literal's `type` is a list that includes `"null"`, e.g. `["integer", "null"]`.
pub(crate) fn is_nullable(ty: Option<&Value>) -> bool {
    ty.and_then(Value::as_array)
        .is_some_and(|types| types.iter().any(|t| t.as_str() == Some("null")))
}

/// Loads a schema the API only allows to be an object (a tool's params, query, body or
/// response). One with no `type` is an object, even when empty; a declared `type` is kept as
/// loaded, so validation can report a schema that isn't an object.
pub(crate) fn container_schema(model: &WireMap) -> Schema {
    let kind = match model.get("type") {
        None | Some(Value::Null) => SchemaKind::Object,
        Some(_) => kind_of(model),
    };
    Schema::from_model(model, false, kind)
}

/// Drops `null` object entries at every level, as the other SDKs' serializers read them as
/// absent. Free-form JSON values (constants, dynamic variable placeholders) are kept as they are,
/// and so is a `null` constant, which `Schema::from_model` keeps for a nullable literal. The
/// values of name maps (`properties`, `path_params_schema`, `request_headers`) are walked as they
/// are, so a param named `constant_value` is no exception.
pub(crate) fn strip_nulls(value: Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .filter(|(key, v)| !v.is_null() || key == "constant_value")
                .map(|(key, v)| {
                    let v = match key.as_str() {
                        "constant_value" | "dynamic_variable_placeholders" => v,
                        "properties" | "path_params_schema" | "request_headers" => match v {
                            Value::Object(names) => Value::Object(
                                names
                                    .into_iter()
                                    .filter(|(_, v)| !v.is_null())
                                    .map(|(name, v)| (name, strip_nulls(v)))
                                    .collect(),
                            ),
                            other => strip_nulls(other),
                        },
                        _ => strip_nulls(v),
                    };
                    (key, v)
                })
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.into_iter().map(strip_nulls).collect()),
        other => other,
    }
}
