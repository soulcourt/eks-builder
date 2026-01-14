use std::fs;
use std::path::Path;
use crate::util::typealias::MetadataLock;

/// Serializes the in-memory lock data as TOML and writes it to `metadata-lock.toml`
/// in the project root.
/// Logs a debug message on success or an error on failure.
///
/// # Arguments
/// * `lock` - The lock data mapping extension slugs to their IDs and version codes.
/// * `project_root` - The root directory where `metadata-lock.toml` should be saved.
pub(crate) fn save(lock: &MetadataLock, project_root: &Path) {
	let path = project_root.join("metadata-lock.toml");

	match toml::to_string(lock) {
		Ok(toml_str) => {
			if let Err(e) = fs::write(&path, toml_str) {
				log::error!("Failed to save lock: {}", e);
			} else {
				log::debug!("Lock saved to {}", path.display());
			}
		}
		Err(e) => log::error!("Failed to serialize lock: {}", e),
	}
}