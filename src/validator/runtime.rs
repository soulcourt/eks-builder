use crate::binary::metadata::RuntimeInfo;
use crate::validator;

pub fn validate_runtime(runtime: &RuntimeInfo) -> Result<(), String> {
    validator::check(runtime.entrypoint.is_none(), "`entrypoint` cannot be empty")?;
    validator::check(runtime.min_api_version < 1, "`min_api_version` must be >= 1")?;

    Ok(())
}