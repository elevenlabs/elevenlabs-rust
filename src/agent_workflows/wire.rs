//! Wire-format helpers. The builder keeps its state in API wire format (snake_case keys, as the
//! REST API and the CLI's `agent_configs/*.json` use them), represented as `serde_json::Value`.

use serde_json::{Map, Value};

/// Sort object keys at every level and write whole floats as integers, so the JSON matches every
/// other SDK byte-for-byte (JavaScript can't tell `300.0` from `300`).
pub fn canonical(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            // BTreeMap-style sorted rebuild (independent of serde_json's preserve_order feature).
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let mut out = Map::new();
            for key in keys {
                out.insert(key.clone(), canonical(&map[key]));
            }
            Value::Object(out)
        }
        Value::Array(items) => Value::Array(items.iter().map(canonical).collect()),
        Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                if n.as_i64().is_none() && n.as_u64().is_none() && f.fract() == 0.0 {
                    if let Some(i) = num_to_i64(f) {
                        return Value::Number(i.into());
                    }
                }
            }
            value.clone()
        }
        other => other.clone(),
    }
}

fn num_to_i64(f: f64) -> Option<i64> {
    // `i64::MAX as f64` rounds up to 2^63, which is one past the range, so the upper bound is
    // exclusive: a value equal to it (e.g. 9223372036854775808.0) would otherwise saturate to
    // i64::MAX on the `as i64` cast and silently change the number. `i64::MIN as f64` is exactly
    // -2^63, so the lower bound stays inclusive.
    if f >= i64::MIN as f64 && f < i64::MAX as f64 {
        Some(f as i64)
    } else {
        None
    }
}

/// Serialize a canonical value with 2-space indentation and a trailing newline: the canonical file
/// format. serde_json's pretty formatter uses 2 spaces, matching Python's `json.dumps(indent=2)`.
pub fn to_json_string(value: &Value) -> String {
    let mut out = serde_json::to_string_pretty(value).expect("wire value serializes");
    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn whole_float_at_i64_max_boundary_is_left_as_float() {
        // 9223372036854775808.0 is 2^63, which equals `i64::MAX as f64` (that cast rounds up past
        // i64::MAX). Casting it to i64 would saturate to 9223372036854775807 and silently change the
        // value, so canonicalization must leave it as a float.
        let two_pow_63 = json!(9_223_372_036_854_775_808.0_f64);
        let out = canonical(&two_pow_63);
        assert!(out.is_f64(), "boundary float must stay a float, got {out}");
        assert_eq!(out.as_f64(), Some(9_223_372_036_854_775_808.0));
    }

    #[test]
    fn in_range_whole_float_still_canonicalizes_to_int() {
        assert_eq!(canonical(&json!(300.0_f64)), json!(300));
    }
}

/// A node/edge model as an object map.
pub type WireMap = Map<String, Value>;

/// Treat a value as an object map, or an empty one.
pub fn as_object(value: &Value) -> WireMap {
    value.as_object().cloned().unwrap_or_default()
}
