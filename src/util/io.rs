use std::{env, fs, io};
use std::path::{Path, PathBuf};
use toml::Value;
use crate::lock::reader;
use crate::util;

pub fn project_root() -> PathBuf {
    env::current_dir()
        .unwrap_or_else(|_| {
            util::exit(1, "Failed to find project root.");
        })
}

pub(crate) fn is_crate(path: &Path) -> bool {
    path.join("Cargo.toml").exists()
}

pub(crate) fn crate_name(extension_path: &Path) -> Option<String> {
    let cargo_toml = extension_path.join("Cargo.toml");

    let contents = fs::read_to_string(cargo_toml).ok()?;

    let value: Value = toml::from_str(&contents).ok()?;

    let name = value
        .get("package")?
        .get("name")?
        .as_str()?;

    Some(name.replace('-', "_"))
}

pub(crate) fn get_slug_by_id(id: &str) -> Option<String> {
    let project_root = project_root();

    let lock = if let Ok(metadata) = reader::content(&project_root) {
        metadata
    } else {
        return None;
    };

    lock.iter().find_map(|(slug, entry)| {
        entry
            .get("id")
            .map(|s| s.as_str() == id)
            .unwrap_or(false)
            .then(|| slug.clone())
    })
}

pub(crate) fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if !src.exists() {
        return Ok(());
    }
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir_recursive(&path, &dest)?;
        } else {
            fs::copy(&path, &dest)?;
        }
    }
    Ok(())
}