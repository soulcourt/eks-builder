use std::{fs, io};
use std::path::Path;
use tiny_http::{Header, Request, Response, Server};
use crate::util;

pub(crate) fn ensure_index_exists(project_root: &Path) -> io::Result<()> {
    let json_file = project_root.join("index.min.json");

    if !json_file.exists() {
        util::exit(1, &format!("File not found: {:?}", json_file));
    }

    Ok(())
}

pub(crate) fn start_server(port: u16) -> io::Result<Server> {
    Server::http(format!("0.0.0.0:{port}"))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub(crate) fn handle_request(request: Request, project_root: &Path) {
    let url_path = request.url().trim_start_matches('/');
    let file_path = project_root.join(url_path);

    let mime = match resolve_mime(&file_path) {
        Some(m) => m,
        None => {
            respond_simple(request, 415, "File type not supported");
            return;
        }
    };

    if !file_path.is_file() {
        respond_simple(request, 404, "File not found");
        return;
    }

    respond_with_file(request, &file_path, mime);
}

fn resolve_mime(path: &Path) -> Option<&'static str> {
    match path.extension().and_then(|e| e.to_str()) {
        Some("json") => Some("application/json"),
        Some("png") => Some("image/png"),
        Some("jpg") | Some("jpeg") => Some("image/jpeg"),
        Some("webp") => Some("image/webp"),
        Some("eks") => Some("application/octet-stream"),
        _ => None,
    }
}

fn respond_simple(request: Request, status: u16, message: &str) {
    let resp = Response::from_string(message).with_status_code(status);
    let _ = request.respond(resp);
}

fn respond_with_file(request: Request, path: &Path, mime: &str) {
    match fs::read(path) {
        Ok(data) => {
            let len = data.len().to_string();

            let response = Response::from_data(data)
                .with_chunked_threshold(usize::MAX)
                .with_header(
                    Header::from_bytes(b"Content-Type", mime.as_bytes()).unwrap(),
                )
                .with_header(
                    Header::from_bytes(b"Content-Length", len.as_bytes()).unwrap(),
                );

            if let Err(e) = request.respond(response) {
                eprintln!("Failed to respond to request: {e}");
            }
        }
        Err(e) => {
            let resp = Response::from_string(format!("Failed to read file: {e}"))
                .with_status_code(500);
            let _ = request.respond(resp);
        }
    }
}