use crate::binary::metadata::ExtensionMetadata;
use std::path::Path;
use figment::Figment;
use figment::providers::{Format, Toml};
use crate::util::typealias::MetadataLock;

/// Determines whether an extension should be rebuilt.
///
/// - The extension is **not present** in the lock file, or
/// - The extension's `version_code` is **greater** than the version in the lock file.
///
/// ## Arguments
/// * `metadata` - Reference to the extension metadata.
/// * `lock_data` - Reference to the lock file data.
///
/// ## Returns
/// `true` if the extension should be rebuilt, `false` otherwise.
pub(crate) fn requires_build(
	metadata: &ExtensionMetadata,
	lock_data: &MetadataLock,
) -> bool {
	match lock_data.get(&metadata.extension.slug) {
		Some(entry) => match entry.get("version_code") {
			Some(v) => metadata.extension.version_code > v.parse::<u32>().unwrap_or(0),
			None => true,
		},
		None => true,
	}
}

/// Retrieves the unique ID of an extension from the lock file.
///
/// ## Arguments
/// * `lock` - Reference to the lock data.
/// * `slug` - Extension slug to look up.
///
/// ## Returns
/// `Some(id)` if the extension exists in the lock, otherwise `None`.
pub(crate) fn get_id(
	lock: &MetadataLock,
	slug: &str,
) -> Option<String> {
	lock.get(slug).and_then(|entry| entry.get("id").cloned())
}

/// Checks the lock against the filesystem and returns extensions that are **missing**.
///
/// ## Arguments
/// * `lock` - Reference to the lock data.
/// * `extensions_root` - Path to the root of installed extensions.
///
/// ## Returns
/// A `Vec<String>` containing the slugs of extensions present in the lock but missing on disk.
///
/// ## Notes
/// - Slugs must follow the format `"lang.extension_name"` (e.g., `"en.realnovels"`).
/// - Invalid slugs are logged as warnings and ignored.
pub(crate) fn find_missing_extensions(
	lock: &MetadataLock,
	extensions_root: &Path,
) -> Vec<String> {
	let mut stale_entries = vec![];

	for slug in lock.keys() {
		let parts: Vec<&str> = slug.split('.').collect();

		if parts.len() < 2 {
			log::warn!("Invalid slug format: {}", slug);
			continue;
		}

		let path = extensions_root.join(parts[0]).join(parts[1]);

		if !path.exists() {
			stale_entries.push(slug.clone());
		}
	}

	stale_entries
}

pub(crate) fn content(project_root: &Path) -> Result<MetadataLock, String> {
	let lock_path = project_root.join("metadata-lock.toml");

	let figment = Figment::new().join(Toml::file(&lock_path));
	let lock: Result<MetadataLock, _> = figment.extract();

	match lock {
		Ok(metadata) => Ok(metadata),
		Err(e) => Err(format!("Failed to parse the TOML content: {}", e)),
	}
}
