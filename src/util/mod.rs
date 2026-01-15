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

pub fn exit(code: i32, msg: &str) -> ! {
	if code == 0 {
		log::info!("{}", msg);
	} else {
		log::error!("{}", msg);
	}

	std::process::exit(code);
}
