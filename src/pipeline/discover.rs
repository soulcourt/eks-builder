use std::fs;
use std::path::{Path, PathBuf};
use crate::binary::metadata::ExtensionMetadata;
use crate::util;
use crate::util::typealias::MetadataLock;

pub fn process_languages(
    extensions_src_dir: &Path,
    project_root: &Path,
    lock: &mut MetadataLock,
    all_metadata: &mut Vec<ExtensionMetadata>,
) {
    for lang in fs::read_dir(extensions_src_dir).unwrap() {
        let lang_path = lang.unwrap().path();

        if !lang_path.is_dir() {
            util::exit(
                1,
                "Only directories are allowed in the extensions source directory",
            );
        }

        process_language(&lang_path, project_root, lock, all_metadata);
    }
}

fn process_language(
    lang_path: &PathBuf,
    project_root: &Path,
    lock: &mut MetadataLock,
    all_metadata: &mut Vec<ExtensionMetadata>,
) {
    for entry in fs::read_dir(lang_path).unwrap() {
        let ext_path = entry.unwrap().path();

        if !ext_path.is_dir() || !util::io::is_crate(&ext_path) {
            continue;
        }

        crate::pipeline::process::process_extension(
            &ext_path,
            project_root,
            lock,
            all_metadata,
        );
    }
}