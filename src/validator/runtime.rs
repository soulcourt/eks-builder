use std::path::Path;
use crate::binary::metadata::RuntimeInfo;
use crate::validator;

pub fn validate_runtime(runtime: &RuntimeInfo) -> Result<(), String> {
    if let Some(entrypoint) = &runtime.entrypoint {
        if !entrypoint.ends_with(".wasm") {
            return Err("`entrypoint` must be a .wasm file".to_string());
        }

        let path = Path::new(entrypoint);

        // Optional: check file exists
        if !path.is_file() {
            return Err(format!("`entrypoint` file not found: {}", entrypoint));
        }

        let data = std::fs::read(path)
            .map_err(|_| format!("Failed to read `entrypoint`: {}", entrypoint))?;

        if data.len() < 4 || &data[0..4] != b"\0asm" {
            return Err(format!("`entrypoint` is not a valid WASM module: {}", entrypoint));
        }
    }

    validator::check(runtime.min_api_version < 1, "`min_api_version` must be >= 1")?;

    Ok(())
}

