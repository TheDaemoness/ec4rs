use std::ffi::OsString;

use clap::Parser;

#[derive(Parser)]
#[clap(disable_version_flag = true)]
struct Args {
	/// Override config filename
	#[clap(default_value = ".editorconfig", short)]
	filename: OsString,
	/// Ignored by this implementation
	#[clap(default_value = ec4rs::EC_VERSION, short = 'b')]
	ec_version: String,
	/// Print test-friendly version information
	#[clap(short, long)]
	version: bool,
	files: Vec<std::path::PathBuf>
}

fn main() {
	let mut args = Args::parse();
	if args.version {
		println!("EditorConfig (ec4rs-parse {}) Version {}", env!("CARGO_PKG_VERSION"), ec4rs::EC_VERSION);
	} else {
		eprintln!("NYI!");
	}
}
