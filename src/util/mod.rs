pub(crate) mod constants;
pub(crate) mod io;
pub(crate) mod typealias;

use uuid::Uuid;

pub(crate) fn generate_uuid() -> String {
	Uuid::new_v4().to_string()
}

pub(crate) fn try_critical<F>(metadata_slug: &str, f: F) -> std::io::Result<()>
where
	F: FnOnce() -> std::io::Result<()>,
{
	f().map_err(|e| {
		log::error!("Skipping extension '{}': {}", metadata_slug, e);
		e
	})
}

pub(crate) fn exit(code: i32, msg: &str, error: bool) -> ! {
	if error {
		log::error!("{}", msg);
	} else {
		log::info!("{}", msg);
	}
	std::process::exit(code);
}
