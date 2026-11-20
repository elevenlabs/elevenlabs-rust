//! File access for `AgentProject::open` and `save`. Files are written owner-only, never through
//! a symlink, and never outside the project.

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::Value;

use super::errors::AgentProjectError;
use super::format::normalize_path;
use super::project::{AGENTS_FILE, AGENT_CONFIGS_DIR, TOOLS_FILE, TOOL_CONFIGS_DIR};

/// Reads the index files and every config they list (branch configs included). Also returns the
/// files already in the config directories, which new files must not overwrite.
pub(crate) fn read_project(
    dir: &Path,
) -> Result<(HashMap<String, String>, Vec<String>), AgentProjectError> {
    let root = fs::canonicalize(dir).map_err(|err| {
        AgentProjectError::new(format!("Can't open project {}: {err}", dir.display()))
    })?;
    let mut files = HashMap::new();
    for index in [AGENTS_FILE, TOOLS_FILE] {
        let Some(text) = read_if_exists(&resolve_in_project(&root, index)?, index)? else {
            continue;
        };
        files.insert(index.to_string(), text.clone());
        // A malformed index is reported by AgentProject, with the parse error.
        let Ok(parsed) = serde_json::from_str::<Value>(&text) else {
            continue;
        };
        let mut configs: Vec<Option<&Value>> = Vec::new();
        for entry in parsed
            .get("agents")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            configs.push(entry.get("config"));
            for branch in entry
                .get("branches")
                .and_then(Value::as_object)
                .into_iter()
                .flat_map(|b| b.values())
            {
                configs.push(branch.get("config"));
            }
        }
        for entry in parsed
            .get("tools")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            configs.push(entry.get("config"));
        }
        for config in configs {
            let Some(config) = config.and_then(Value::as_str) else {
                return Err(AgentProjectError::new(format!(
                    "{index} has an entry without a \"config\" path"
                )));
            };
            if let Some(content) = read_if_exists(&resolve_in_project(&root, config)?, config)? {
                files.insert(normalize_path(config), content);
            }
        }
    }
    let mut taken = Vec::new();
    for sub in [AGENT_CONFIGS_DIR, TOOL_CONFIGS_DIR] {
        if let Ok(entries) = fs::read_dir(root.join(sub)) {
            for entry in entries.flatten() {
                taken.push(format!("{sub}/{}", entry.file_name().to_string_lossy()));
            }
        }
    }
    Ok((files, taken))
}

/// Writes each file owner-only (`0600`), creating directories as needed. Each file is written to
/// a new temporary file and renamed into place, so a symlink at the target is never followed; a
/// target that is a symlink is refused outright.
pub(crate) fn write_project_files(
    dir: &Path,
    files: &[(String, String)],
) -> Result<(), AgentProjectError> {
    let root = fs::canonicalize(dir).map_err(|err| {
        AgentProjectError::new(format!("Can't open project {}: {err}", dir.display()))
    })?;
    for (path, text) in files {
        let target = resolve_in_project(&root, path)?;
        let io = |err: std::io::Error| AgentProjectError::new(format!("Can't write {path}: {err}"));
        let parent = target
            .parent()
            .ok_or_else(|| AgentProjectError::new(format!("Can't write {path}")))?;
        fs::create_dir_all(parent).map_err(io)?;
        // The directories may be new: check again that none of them leads outside.
        resolve_in_project(&root, path)?;
        if fs::symlink_metadata(&target).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err(AgentProjectError::new(format!(
                "Refusing to write {path}: it's a symlink"
            )));
        }
        let temp = parent.join(temp_name(&target));
        let written = write_new_file(&temp, text).and_then(|()| fs::rename(&temp, &target));
        if let Err(err) = written {
            let _ = fs::remove_file(&temp);
            return Err(io(err));
        }
    }
    Ok(())
}

fn temp_name(target: &Path) -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let name = target
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    format!(
        ".{name}.{}.{}.tmp",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    )
}

/// Creates `path`, which must not exist yet (so it can't be a symlink), owner-only.
fn write_new_file(path: &Path, text: &str) -> std::io::Result<()> {
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path)?;
    file.write_all(text.as_bytes())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(fs::Permissions::from_mode(0o600))?;
    }
    Ok(())
}

/// The CLI's `resolve_in_project`: the absolute path for a project-relative one, refusing
/// anything that escapes the project, including through a symlinked directory.
pub(crate) fn resolve_in_project(root: &Path, path: &str) -> Result<PathBuf, AgentProjectError> {
    if path.trim().is_empty() || path.chars().any(char::is_control) {
        return Err(AgentProjectError::new(format!(
            "Invalid config path {}",
            Value::String(path.to_string())
        )));
    }
    let mut candidate = PathBuf::new();
    for component in root.join(path).components() {
        match component {
            Component::ParentDir => {
                candidate.pop();
            }
            Component::CurDir => {}
            other => candidate.push(other),
        }
    }
    let inside = |p: &Path| p.starts_with(root);
    if !inside(&candidate) {
        return Err(AgentProjectError::new(format!(
            "Config path {path} is outside the project"
        )));
    }
    // The deepest part of the path that exists, with symlinks resolved, must still be inside.
    let mut ancestor = candidate.as_path();
    loop {
        match fs::canonicalize(ancestor) {
            Ok(real) => {
                if !inside(&real) {
                    return Err(AgentProjectError::new(format!(
                        "Config path {path} leads outside the project through a symlink"
                    )));
                }
                return Ok(candidate);
            }
            Err(_) => match ancestor.parent() {
                Some(parent) => ancestor = parent,
                None => {
                    return Err(AgentProjectError::new(format!(
                        "Config path {path} is outside the project"
                    )))
                }
            },
        }
    }
}

fn read_if_exists(path: &Path, label: &str) -> Result<Option<String>, AgentProjectError> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(AgentProjectError::new(format!("Can't read {label}: {err}"))),
    }
}
