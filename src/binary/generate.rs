use std::io;
use crate::binary::payload;
use crate::binary::payload::create_directories;
use crate::{lock, util};
use std::path::{Path};
use std::process::Command;
use crate::binary::metadata::ExtensionMetadata;
use crate::util::constants;
use crate::util::typealias::MetadataLock;

pub(crate) fn extension_binary(
	extension_path: &Path,
	metadata: &ExtensionMetadata,
	project_root: &Path,
	lock: &MetadataLock,
) -> io::Result<()> {
	let build_dir = extension_path.join(constants::TARGET_DIR);
	let payload_dir = build_dir.join("payload");

	let metadata_slug =  metadata.extension.slug.as_str();
	let metadata_entrypoint =  metadata.runtime.entrypoint.clone();

	let id = lock::reader::get_id(lock, metadata_slug)
		.ok_or_else(|| io::Error::other(format!("No ID found for slug '{}'", metadata_slug)))?;

	let output_eks = project_root
		.join("extensions")
		.join(&id)
		.join(constants::EXTENSION_BINARY);

	create_directories(&payload_dir, &output_eks);

	// --- Payload contents ---
	payload::copy_assets(extension_path, &payload_dir);
	payload::copy_resource(constants::METADATA_FILE, extension_path, &payload_dir);
	util::try_critical(metadata_slug, || {
		payload::create_extension_icon(extension_path, &payload_dir, project_root, &id)
	})?;
	util::try_critical(metadata_slug, || {
		payload::copy_wasm(&payload_dir, &build_dir, extension_path, metadata_entrypoint)
	})?;

	let zip_status = Command::new("zip")
	    .arg("-qr")
	    .arg(&output_eks)
	    .arg(".")
	    .current_dir(&payload_dir)
	    .status()
	    .expect("Failed to run zip command");

	if zip_status.success() {
		Ok(())
	} else {
		Err(io::Error::other("zip failed"))
	}
}