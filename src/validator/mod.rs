use std::path::Path;
use crate::binary::metadata::ExtensionMetadata;
use crate::validator::api::validate_api;
use crate::validator::extension::validate_extension;
use crate::validator::runtime::validate_runtime;

mod extension;
mod api;
mod runtime;

pub(crate) fn validate_metadata(ext_path: &Path, project_root: &Path) -> Option<ExtensionMetadata> {
    let display_path = ext_path.strip_prefix(project_root.join("src")).unwrap();

    let metadata = match ExtensionMetadata::load(&ext_path.join("res/metadata.toml")) {
        Ok(m) => m,
        Err(err) => {
            log::warn!("Skipping {:?}: failed to load metadata: {}", display_path, err);
            return None;
        }
    };

    if let Err(err) = validate_extension(&metadata.extension) {
        log::warn!("Skipping {:?}: invalid extension info: {}", display_path, err);
        return None;
    }

    if let Err(err) = validate_api(&metadata.api) {
        log::warn!("Skipping {:?}: invalid api info: {}", display_path, err);
        return None;
    }

    if let Err(err) = validate_runtime(&metadata.runtime) {
        log::warn!("Skipping {:?}: invalid runtime info: {}", display_path, err);
        return None;
    }

    Some(metadata)
}

pub(crate) fn check(condition: bool, message: &str) -> Result<(), String> {
    if condition {
        return Err(message.to_string());
    }
    Ok(())
}