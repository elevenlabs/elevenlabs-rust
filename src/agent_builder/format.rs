//! File naming in the CLI's project layout.

use std::collections::HashSet;

/// The CLI's `sanitize_filename`: a filesystem-safe file stem for an agent or tool name.
/// `/ \ : * ? " < > |` and whitespace become `-`, runs of `-` collapse, leading and trailing `.`
/// and `-` are stripped, a name that started with `.` gets a `_` prefix, an empty result becomes
/// `unnamed`, and names are cut at 100 characters.
pub fn sanitize_filename(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return "unnamed".into();
    }
    let mut sanitized: String = trimmed
        .chars()
        .map(|c| {
            if "/\\:*?\"<>|".contains(c) || c.is_whitespace() {
                '-'
            } else {
                c
            }
        })
        .collect();
    while sanitized.contains("--") {
        sanitized = sanitized.replace("--", "-");
    }
    let stripped = sanitized.trim_matches(|c| c == '.' || c == '-');
    if stripped.is_empty() {
        return "unnamed".into();
    }
    let result = if trimmed.starts_with('.') {
        format!("_{stripped}")
    } else {
        stripped.to_string()
    };
    result.chars().take(100).collect()
}

/// The CLI's `generate_unique_filename`: `<dir>/<name>.json`, then `-1`, `-2`... while the path
/// is taken.
pub fn unique_path(dir: &str, name: &str, taken: &HashSet<String>) -> String {
    let stem = sanitize_filename(name);
    let mut candidate = format!("{dir}/{stem}.json");
    let mut n = 1;
    while taken.contains(&candidate) {
        candidate = format!("{dir}/{stem}-{n}.json");
        n += 1;
    }
    candidate
}

/// A project-relative path in one canonical spelling: forward slashes, no `./` or empty
/// segments.
pub(crate) fn normalize_path(path: &str) -> String {
    path.replace('\\', "/")
        .split('/')
        .filter(|part| !part.is_empty() && *part != ".")
        .collect::<Vec<_>>()
        .join("/")
}
