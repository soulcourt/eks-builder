use std::path::Path;
use std::{fs, io};
use tiny_http::{Header, Response, Server};

mod binary;
pub mod cli;
mod lock;
mod pipeline;
mod server;
pub mod util;
mod validator;

pub fn run(project_root: &Path) -> Result<(), io::Error> {
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

pub fn serve(project_root: &Path, port: u16) -> io::Result<()> {
	let json_file = project_root.join("index.min.json");
	if !json_file.exists() {
		util::exit(1, &format!("File not found: {:?}", json_file));
	}

	let json_content = fs::read(&json_file)?;

	let server = Server::http(format!("0.0.0.0:{}", port))
		.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

	for request in server.incoming_requests() {
		// Serve the JSON file for every request
		let response = Response::from_data(json_content.clone())
			.with_header(
				"Content-Type: application/json".parse::<Header>().unwrap()
			);

		if let Err(e) = request.respond(response) {
			log::warn!("Failed to respond to request: {}", e);
		}
	}

	Ok(())
}
