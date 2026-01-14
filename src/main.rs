fn main() {
	env_logger::init();

	let project_root = eks_builder::util::io::project_root();

	log::info!("Building project {:?}", project_root);

	if let Err(e) = eks_builder::run(&project_root) {
		eks_builder::util::exit(1, &format!("Error: {}", e));
	}
}