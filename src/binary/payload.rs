use crate::util;
use crate::util::constants;
use image::{imageops::FilterType, DynamicImage};
use log;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub(crate) fn create_directories(payload_dir: &PathBuf, output_eks: &Path) {
	if let Some(parent) = output_eks.parent() {
		if let Err(e) = fs::create_dir_all(parent) {
			log::error!("Failed to create output directory: {}", e);
			return;
		}
	}

	if let Err(e) = fs::create_dir_all(payload_dir) {
		log::error!("Failed to create Payload directory: {}", e);
	}
}

pub(crate) fn copy_assets(extension_path: &Path, payload_dir: &Path) {
	let assets_dir = extension_path.join("assets");

	if assets_dir.exists() {
		if let Err(e) = util::io::copy_dir_recursive(&assets_dir, &payload_dir.join("assets")) {
			log::warn!("Failed to copy assets/: {}", e);
		}
	}
}

pub(crate) fn copy_resource(filename: &str, extension_path: &Path, payload_dir: &Path) {
	let res_dir = payload_dir.join("res");

	if !res_dir.exists() {
		if let Err(e) = fs::create_dir_all(&res_dir) {
			log::error!("Failed to create res directory {:?}: {}", res_dir, e);
		}
	}

	let src = extension_path.join("res").join(filename);
	let dst = payload_dir.join(filename);

	if let Err(e) = fs::copy(&src, &dst) {
		log::warn!("Failed to copy {}: {}", filename, e);
	}
}

pub(crate) fn copy_wasm(
	payload_dir: &Path,
	build_dir: &Path,
	extension_path: &Path,
	entrypoint: Option<String>,
) -> io::Result<()> {
	let crate_name = util::io::crate_name(extension_path)
		.ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Failed to determine crate name"))?;

	let wasm_src = build_dir.join(format!("{crate_name}.wasm"));

	if !wasm_src.exists() {
		return Err(io::Error::new(
			io::ErrorKind::NotFound,
			format!("Expected wasm file not found: {:?}", wasm_src),
		));
	}

	let wasm_dst =
		payload_dir.join(entrypoint.unwrap_or_else(|| constants::ENTRY_FILE.parse().unwrap()));

	fs::copy(&wasm_src, &wasm_dst).map(|_| ())
}

pub(crate) fn create_extension_icon(
	extension_path: &Path,
	payload_dir: &Path,
	project_root: &Path,
	id: &str,
) -> io::Result<()> {
	let ext_slug = util::io::get_slug_by_id(id).unwrap_or(id.to_string());

	let icon_path = find_icon(extension_path, &ext_slug).ok_or_else(|| {
		io::Error::new(
			io::ErrorKind::NotFound,
			format!("No icon found for slug '{}'", ext_slug),
		)
	})?;

	generate_density_icons(&icon_path, payload_dir, &ext_slug)?;

	if let Err(e) = copy_icon_for_server(&icon_path, &project_root, id) {
		log::error!("Failed to copy server icon for '{}': {}", ext_slug, e);
	}

	Ok(())
}

fn find_icon(extension_path: &Path, id: &str) -> Option<PathBuf> {
	let extensions = ["png", "webp", "svg", "jpg"];
	let res_dir = extension_path.join("res");

	let icon_path = extensions
		.iter()
		.map(|ext| res_dir.join(format!("icon.{}", ext)))
		.find(|path| path.exists());

	if icon_path.is_none() {
		log::warn!("No icon found for '{}'. Tried {:?}", id, extensions);
	}

	icon_path
}

fn copy_icon_for_server(icon_src: &Path, project_root: &Path, id: &str) -> io::Result<u64> {
	let icon_dst = project_root.join("extensions").join(id).join(
		icon_src
			.file_name()
			.unwrap()
			.to_str()
			.unwrap()
			.replace("icon", "preview"),
	);

	fs::copy(icon_src, &icon_dst)
}

fn generate_density_icons(icon_src: &Path, payload_dir: &Path, ext_slug: &str) -> io::Result<()> {
	let out_dir = payload_dir.join("res");

	let image = image::open(icon_src).map_err(|e| {
		log::error!("Required icon missing or unreadable '{}': {}", ext_slug, e);
		io::Error::other(e)
	})?;

	let base_name = "icon";
	let ext = icon_src
		.extension()
		.unwrap_or_else(|| std::ffi::OsStr::new("png"))
		.to_string_lossy();

	let sizes = [(192, ""), (128, "@2x"), (64, "@3x")];

	for (size, suffix) in sizes {
		let out_file = out_dir.join(format!("{}{}.{}", base_name, suffix, ext));
		save_resized(&image, &out_file, size)?;
	}

	Ok(())
}

pub fn save_resized(image: &DynamicImage, path: &Path, size: u32) -> io::Result<()> {
	image
		.resize_exact(size, size, FilterType::Lanczos3)
		.save(path)
		.map_err(io::Error::other)
}
