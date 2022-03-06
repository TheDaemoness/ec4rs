use std::ffi::OsString;

use clap::Parser;

#[derive(Parser)]
#[clap(disable_version_flag = true)]
struct Args {
	/// Override config filename
	#[clap(short)]
	filename: Option<OsString>,
	/// Ignored by this implementation
	#[clap(default_value = ec4rs::version::STRING, short = 'b')]
	ec_version: String,
	/// Print test-friendly version information
	#[clap(short, long)]
	version: bool,
	files: Vec<std::path::PathBuf>
}

fn print_config(path: &std::path::Path, filename: Option<&OsString>) {
	match ec4rs::get_config_for(path, filename) {
		Ok(props) => {
			for (key, value) in props.iter_raw() {
				println!("{}={}", key, value);
			}
		}
		Err(e) => eprintln!("{e}")
	}
}

fn main() {
	let args = Args::parse();
	if args.version {
		println!("EditorConfig (ec4rs-parse {}) Version {}", env!("CARGO_PKG_VERSION"), ec4rs::version::STRING);
	} else if args.files.len() == 1 {
		print_config(args.files.first().unwrap(), args.filename.as_ref());
	} else {
		for path in args.files {
			println!("[{}]", path.to_string_lossy());
			print_config(&path, args.filename.as_ref());
		}
	}
}
