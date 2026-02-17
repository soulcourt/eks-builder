use std::path::Path;
use std::io;

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

    server::index::generate_server_index(project_root, &all_metadata, &lock);

    Ok(())
}

pub fn serve(project_root: &Path, port: u16) -> io::Result<()> {
    server::http::ensure_index_exists(project_root)?;

    println!(
        "Serving {} on http://0.0.0.0:{}/",
        project_root.display(),
        port
    );

    let server = server::http::start_server(port)?;

    for request in server.incoming_requests() {
        server::http::handle_request(request, project_root);
    }

    Ok(())
}
