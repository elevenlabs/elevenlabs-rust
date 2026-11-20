//! The canonical file format, byte-for-byte as the JS SDK writes it.
//!
//! `serde_json` writes the same structure and strings as `JSON.stringify(value, null, 2)`, but not
//! the same numbers: ryu writes `1e-7` and `1e21` where JavaScript writes `1e-7` and `1e+21`, and
//! `1.2345678901234568e20` where JavaScript writes `123456789012345680000`. Non-integer numbers are
//! therefore written as ECMAScript's `Number.prototype.toString` writes them.

use std::io;

use serde::Serialize;
use serde_json::ser::{Formatter, PrettyFormatter, Serializer};
use serde_json::Value;

/// `value` with its object keys sorted at every level: the shape `to_json()` returns. A whole
/// float becomes an integer (`3.0` is `3`, as in JavaScript) only when the integer has the digits
/// JavaScript writes for it. Otherwise it stays an f64, so [`to_json_string`] writes it as
/// JavaScript does: `1000000000000000100.0` is written `1000000000000000100`, where the nearest
/// i64 (the workflow builder's `canonical()`) would be written `1000000000000000128`.
pub(crate) fn js_canonical(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            Value::Object(
                keys.into_iter()
                    .map(|k| (k.clone(), js_canonical(&map[k])))
                    .collect(),
            )
        }
        Value::Array(items) => Value::Array(items.iter().map(js_canonical).collect()),
        Value::Number(n) if n.is_f64() => {
            let f = n.as_f64().expect("an f64 number");
            whole_as_integer(f).unwrap_or_else(|| value.clone())
        }
        other => other.clone(),
    }
}

/// `f` as an i64 or u64 number, if it is whole and that integer reads the same as `js_number(f)`.
fn whole_as_integer(f: f64) -> Option<Value> {
    if !f.is_finite() || f.fract() != 0.0 {
        return None;
    }
    // 2^63 and 2^64: `as` saturates at the bounds, so they are exclusive.
    let (integer, digits): (Value, String) =
        if (-9223372036854775808.0..9223372036854775808.0).contains(&f) {
            ((f as i64).into(), (f as i64).to_string())
        } else if (0.0..18446744073709551616.0).contains(&f) {
            ((f as u64).into(), (f as u64).to_string())
        } else {
            return None;
        };
    (digits == js_number(f)).then_some(integer)
}

/// `value` with 2-space indentation and a trailing newline, numbers formatted as in JavaScript.
pub(crate) fn to_json_string(value: &Value) -> String {
    let mut out = to_json_string_indented(value, b"  ");
    out.push('\n');
    out
}

/// `value` pretty-printed with `indent` and no trailing newline, numbers formatted as in
/// JavaScript: `JSON.stringify(value, null, indent)`. The agent project files use 4 spaces.
pub(crate) fn to_json_string_indented(value: &Value, indent: &[u8]) -> String {
    let mut out = Vec::new();
    let mut serializer = Serializer::with_formatter(
        &mut out,
        JsFormatter(PrettyFormatter::with_indent(indent)),
    );
    value
        .serialize(&mut serializer)
        .expect("a JSON value serializes");
    String::from_utf8(out).expect("serde_json writes UTF-8")
}

/// `f` as ECMAScript's `Number::toString(f)` writes it (radix 10), for a finite `f`.
pub(crate) fn js_number(f: f64) -> String {
    if f == 0.0 {
        // -0 too.
        return "0".into();
    }
    // Rust's shortest round-trip digits, as JavaScript chooses them: `d[.ddd]e<exp>`.
    let scientific = format!("{:e}", f.abs());
    let (mantissa, exponent) = scientific
        .split_once('e')
        .expect("`{:e}` writes an exponent");
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    let exponent: i32 = exponent.parse().expect("`{:e}` writes an integer exponent");
    // In the spec's terms: the value is digits × 10^(n − k), with k digits.
    let k = digits.len() as i32;
    let n = exponent + 1;
    let body = if k <= n && n <= 21 {
        format!("{digits}{}", "0".repeat((n - k) as usize))
    } else if 0 < n && n <= 21 {
        let (whole, fraction) = digits.split_at(n as usize);
        format!("{whole}.{fraction}")
    } else if -6 < n && n <= 0 {
        format!("0.{}{digits}", "0".repeat((-n) as usize))
    } else {
        let sign = if n - 1 < 0 { '-' } else { '+' };
        let (first, rest) = digits.split_at(1);
        let mantissa = if rest.is_empty() {
            first.to_string()
        } else {
            format!("{first}.{rest}")
        };
        format!("{mantissa}e{sign}{}", (n - 1).abs())
    };
    if f < 0.0 {
        format!("-{body}")
    } else {
        body
    }
}

/// serde_json's pretty formatter (2-space indentation), with JavaScript's float formatting.
struct JsFormatter<'a>(PrettyFormatter<'a>);

impl Formatter for JsFormatter<'_> {
    fn write_f64<W: ?Sized + io::Write>(&mut self, writer: &mut W, value: f64) -> io::Result<()> {
        writer.write_all(js_number(value).as_bytes())
    }

    fn write_f32<W: ?Sized + io::Write>(&mut self, writer: &mut W, value: f32) -> io::Result<()> {
        self.write_f64(writer, f64::from(value))
    }

    fn begin_array<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.0.begin_array(writer)
    }

    fn end_array<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.0.end_array(writer)
    }

    fn begin_array_value<W: ?Sized + io::Write>(
        &mut self,
        writer: &mut W,
        first: bool,
    ) -> io::Result<()> {
        self.0.begin_array_value(writer, first)
    }

    fn end_array_value<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.0.end_array_value(writer)
    }

    fn begin_object<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.0.begin_object(writer)
    }

    fn end_object<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.0.end_object(writer)
    }

    fn begin_object_key<W: ?Sized + io::Write>(
        &mut self,
        writer: &mut W,
        first: bool,
    ) -> io::Result<()> {
        self.0.begin_object_key(writer, first)
    }

    fn begin_object_value<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.0.begin_object_value(writer)
    }

    fn end_object_value<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.0.end_object_value(writer)
    }
}

#[cfg(test)]
mod tests {
    use super::{js_number, to_json_string};
    use serde_json::json;

    #[test]
    fn formats_numbers_as_javascript_does() {
        // Each expected string is what `String(x)` / `JSON.stringify(x)` gives in JavaScript.
        for (value, js) in [
            (1e-7, "1e-7"),
            (1.5e-7, "1.5e-7"),
            (0.000001, "0.000001"),
            (0.0000015, "0.0000015"),
            (1e21, "1e+21"),
            (1.5e21, "1.5e+21"),
            (123456789012345680000.0, "123456789012345680000"),
            (1e19, "10000000000000000000"),
            (1e20, "100000000000000000000"),
            (1.5, "1.5"),
            (0.1, "0.1"),
            (-0.5, "-0.5"),
            (-1e-7, "-1e-7"),
            (-1e21, "-1e+21"),
            (123.456, "123.456"),
            (1e300, "1e+300"),
            (5e-324, "5e-324"),
            (1.7976931348623157e308, "1.7976931348623157e+308"),
            (-0.0, "0"),
        ] {
            assert_eq!(js_number(value), js, "{value:e}");
        }
    }

    #[test]
    fn writes_the_canonical_file_format() {
        let value = json!({ "a": [1e-7, 1e21, 2], "b": { "c": 0.000001 }, "d": [], "e": {} });
        assert_eq!(
            to_json_string(&value),
            "{\n  \"a\": [\n    1e-7,\n    1e+21,\n    2\n  ],\n  \"b\": {\n    \"c\": 0.000001\n  },\n  \"d\": [],\n  \"e\": {}\n}\n"
        );
        // Everything but numbers is as serde_json writes it.
        let plain = json!({ "s": "é\"\n", "n": null, "t": true, "i": -3 });
        assert_eq!(
            to_json_string(&plain),
            format!("{}\n", serde_json::to_string_pretty(&plain).unwrap())
        );
    }
}
