use lock::{manager, reader, writer};

mod binary;
mod lock;
mod server;
mod util;
mod validator;
mod pipeline;

fn main() {
	env_logger::init();

	let project_root = util::io::project_root();
	let extensions_src_dir = project_root.join("src");

	if !extensions_src_dir.exists() {
		util::exit(1, "Extensions source directory does not exist.", true);
	}

	let mut lock = match reader::content(&project_root) {
		Ok(metadata) => metadata,
		Err(_) => util::exit(1, "Error loading metadata lock", true)
	};

	let mut all_metadata = Vec::new();

	pipeline::discover::process_languages(
		&extensions_src_dir,
		&project_root,
		&mut lock,
		&mut all_metadata,
	);

	let stale = reader::find_missing_extensions(&lock, &extensions_src_dir);
	if !stale.is_empty() {
		manager::remove_stale(&mut lock, &stale);
        writer::save(&lock, &project_root)
	}

	server::generate_server_index(&project_root, &all_metadata, &lock);
}
