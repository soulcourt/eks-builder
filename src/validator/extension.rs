use url::Url;
use crate::binary::metadata::ExtensionInfo;
use crate::validator;

pub fn validate_extension(extension: &ExtensionInfo) -> Result<(), String> {
    validator::check(extension.name.trim().is_empty(), "`name` cannot be empty")?;
    validator::check(extension.slug.trim().is_empty(), "`slug` cannot be empty")?;
    validator::check(extension.base_url.trim().is_empty(), "`base_url` cannot be empty")?;
    validator::check(extension.version_name.trim().is_empty(), "`version_name` cannot be empty")?;
    validator::check(extension.version_code < 1, "`version_code` must be greater than 0")?;
    validator::check(extension.language.trim().is_empty(), "`language` cannot be empty")?;

    // Validate URL format
    validator::check(
        Url::parse(&extension.base_url).is_err(),
        "`base_url` must be a valid URL"
    )?;

    Ok(())
}