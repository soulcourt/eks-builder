use std::path::{Path, PathBuf};
use crate::{
    validator,
    binary,
    pipeline::build,
};
use crate::binary::metadata::ExtensionMetadata;
use crate::util::typealias::MetadataLock;

pub fn process_extension(
    ext_path: &PathBuf,
    project_root: &Path,
    lock: &mut MetadataLock,
    all_metadata: &mut Vec<ExtensionMetadata>,
) {
    let metadata = match validator::validate_metadata(ext_path, project_root) {
        Some(m) => m,
        None => return,
    };

    all_metadata.push(metadata.clone());

    if build::should_build(&metadata, project_root, lock) && build::build_extension(ext_path) {
        if let Err(e) = binary::generate::extension_binary(ext_path, &metadata, project_root, lock) {
            log::error!(
            "Failed to generate binary for '{}': {}",
            metadata.extension.slug,
            e
        );
            return;
        }
    }
}