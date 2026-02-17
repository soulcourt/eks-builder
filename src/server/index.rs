use std::fs;
use std::path::Path;
use crate::binary::metadata::ExtensionMetadata;
use crate::lock;
use crate::server::data::ServerItem;
use crate::util::constants;
use crate::util::typealias::MetadataLock;

pub(crate) fn generate_server_index(
    project_root: &Path,
    all_metadata: &Vec<ExtensionMetadata>,
    lock: &MetadataLock,
) {
    let mut index_list = vec![];

    for metadata in all_metadata {
        let id = match lock::reader::get_id(lock, &metadata.extension.slug) {
            Some(id) => id.to_string(),
            None => {
                log::warn!(
					"Skipping {} — no ID found in metadata-lock",
					metadata.extension.slug
				);
                continue;
            }
        };

        let extension = &metadata.extension;

        let index = ServerItem {
            id: id.clone(),
            name: extension.name.clone(),
            slug: extension.slug.clone(),
            language: extension.language.clone(),
            version_code: extension.version_code,
            version_name: extension.version_name.clone(),
        };

        index_list.push(index);
    }

    let output_pretty = project_root.join(constants::SERVER_INDEX);
    let output_min = project_root.join(constants::SERVER_INDEX_MINIFIED);

    create_index_file(&output_pretty, &index_list, true);
    create_index_file(&output_min, &index_list, false);
}

fn create_index_file(output: &Path, index_list: &Vec<ServerItem>, pretty: bool) {
    let title =  if pretty { "📘Prettified" } else { "📗Minified" };

    let json = if pretty {
        match serde_json::to_string_pretty(&index_list) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to serialize {title}: {}", e);
                return;
            }
        }
    } else {
        match serde_json::to_string(&index_list) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to serialize {title}: {}", e);
                return;
            }
        }
    };

    if let Err(e) = fs::write(output, json) {
        log::error!("Failed to write {title}.json: {}", e);
    } else {
        log::info!("{title} written to {}", output.display());
    }
}