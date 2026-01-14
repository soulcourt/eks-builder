use serde::Deserialize;
use std::path::Path;
use std::{fs, io};

#[derive(Debug, Deserialize, Clone)]
pub struct ExtensionMetadata {
	pub extension: ExtensionInfo,
	pub api: ApiInfo,
	pub runtime: RuntimeInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExtensionInfo {
	pub slug: String,
	pub name: String,
	pub version_code: u32,
	pub version_name: String,
	pub language: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApiInfo {
	pub version: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RuntimeInfo {
	pub entrypoint: Option<String>,
	pub min_api_version: u32,
}

impl ExtensionMetadata {
	pub fn load(path: &Path) -> io::Result<Self> {
		let content = fs::read_to_string(path)?;
		toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
	}
}
