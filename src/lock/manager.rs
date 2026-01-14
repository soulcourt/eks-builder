use crate::binary::metadata::ExtensionMetadata;
use crate::util;
use std::collections::HashMap;
use crate::util::typealias::MetadataLock;

/// Adds a new entry for an extension in the lock file.
/// If the extension already exists, the function does nothing.
///
/// ## Arguments
/// * `lock` - Mutable reference to the lock data.
/// * `metadata` - Reference to the extension metadata.
pub(crate) fn add(
	lock: &mut MetadataLock,
	metadata: &ExtensionMetadata,
) {
	if lock.contains_key(&metadata.extension.slug) {
		return;
	}

	let mut entry = HashMap::new();
	entry.insert("id".to_string(), util::generate_uuid());
	entry.insert(
		"version_code".to_string(),
		metadata.extension.version_code.to_string(),
	);

	lock.insert(metadata.extension.slug.clone(), entry);
	log::debug!("Added '{}' to lock", metadata.extension.slug);
}

/// Updates the `version_code` in the lock for an existing extension.
///
/// ## Arguments
/// * `lock` - Mutable reference to the lock data.
/// * `metadata` - Reference to the extension metadata.
///
/// ## Behavior
/// - If the extension exists, updates its `version_code` and logs the update.
/// - If the extension does not exist, logs an error.
pub(crate) fn update_version(
	lock: &mut HashMap<String, HashMap<String, String>>,
	metadata: &ExtensionMetadata,
) {
	if let Some(entry) = lock.get_mut(&metadata.extension.slug) {
		entry.insert(
			"version_code".to_string(),
			metadata.extension.version_code.to_string(),
		);
		log::info!(
			"Updated '{}' version_code to {}",
			metadata.extension.slug,
			metadata.extension.version_code
		);
	} else {
		log::error!(
			"Cannot update version_code: '{}' not found",
			metadata.extension.slug
		);
	}
}

/// Removes extensions from the lock file that are no longer present on disk.
///
/// ## Arguments
/// * `lock` - Mutable reference to the lock data.
/// * `slugs` - Slice of extension slugs to remove.
pub(crate) fn remove_stale(lock: &mut HashMap<String, HashMap<String, String>>, slugs: &[String]) {
	for slug in slugs {
		lock.remove(slug);
		log::info!("Removed stale lock entry: {}", slug);
	}
}
