use crate::binary::metadata::ApiInfo;
use crate::validator;

pub fn validate_api(api: &ApiInfo) -> Result<(), String> {
    validator::check(api.version < 1, "API version must be greater than 0.")?;

    Ok(())
}