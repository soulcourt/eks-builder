use clap::{Parser};
use eks_builder::{cli, cli::{Commands}, util};

fn main() {
	env_logger::init();

	let args = cli::Args::parse();

	unsafe {
		std::env::set_var(
			"RUST_LOG",
			match args.verbose {
				0 => "info",
				1 => "debug",
				_ => "trace",
			},
		);
	}

	if let Err(e) = match args.command {
		Commands::Build { path } => eks_builder::run(&path),
		Commands::Serve { path, port } => eks_builder::serve(&path, port),
	} {
		util::exit(1, &format!("Error: {}", e));
	}
}
