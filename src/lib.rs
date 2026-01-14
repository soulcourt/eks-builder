mod binary;
mod lock;
mod server;
pub mod util;
mod validator;
mod pipeline;

pub fn run(project_root: &std::path::Path) -> Result<(), std::io::Error> {
	let extensions_src_dir = project_root.join("src");

	if !extensions_src_dir.exists() {
		util::exit(1, "Extensions source directory does not exist.");
	}

	let mut lock = match lock::reader::content(project_root) {
		Ok(metadata) => metadata,
		Err(_) => util::exit(1, "Error loading metadata lock"),
	};

	let mut all_metadata = Vec::new();

	pipeline::discover::process_languages(
		&extensions_src_dir,
		project_root,
		&mut lock,
		&mut all_metadata,
	);

	let stale = lock::reader::find_missing_extensions(&lock, &extensions_src_dir);
	if !stale.is_empty() {
		lock::manager::remove_stale(&mut lock, &stale);
		lock::writer::save(&lock, project_root);
	}

	server::generate_server_index(project_root, &all_metadata, &lock);

	Ok(())
}