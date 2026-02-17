use std::path::{Path, PathBuf};
use std::process::Command;
use crate::binary::metadata::ExtensionMetadata;
use crate::lock::{reader, manager, writer};
use crate::util::typealias::MetadataLock;

pub(crate) fn should_build(
    metadata: &ExtensionMetadata,
    project_root: &Path,
    lock: &mut MetadataLock,
) -> bool {
    if lock.contains_key(&metadata.extension.slug) {
        if reader::requires_build(metadata, lock) {
            manager::update_version(lock, metadata);
            writer::save(lock, project_root);
            true
        } else {
            true
        }
    } else {
        manager::add(lock, metadata);
        writer::save(lock, project_root);
        true
    }
}

pub(crate) fn build_extension(extension_path: &PathBuf) -> bool {
    let manifest = extension_path.join("Cargo.toml");
    let target_dir = extension_path.join("target");

    let status = Command::new("cargo")
        .arg("+nightly")
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--manifest-path")
        .arg(&manifest)
        .env("CARGO_TARGET_DIR", &target_dir)
        .status()
        .expect("Failed to run cargo build");

    status.success()
}