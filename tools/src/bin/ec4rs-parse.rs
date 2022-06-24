use std::path::PathBuf;

use clap::Parser;
use semver::{Version, VersionReq};

#[derive(Parser)]
#[clap(disable_version_flag = true)]
struct Args {
	/// Override config filename
	#[clap(short)]
	filename: Option<PathBuf>,
	/// Mostly ignored by this implementation
	#[clap(default_value = ec4rs::version::STRING, short = 'b')]
	ec_version: Version,
	/// Print test-friendly version information
	#[clap(short, long)]
	version: bool,
	files: Vec<PathBuf>
}

fn print_config(
	path: &std::path::Path,
	filename: Option<&PathBuf>,
	legacy_fallbacks: bool,
) {
	match ec4rs::properties_from_config_of(path, filename) {
		Ok(mut props) => {
			if legacy_fallbacks {
				props.use_fallbacks_legacy();
			} else {
				props.use_fallbacks();
			}
			for (key, value) in props.iter() {
				let value = value.into_str();
				if ec4rs::property::STANDARD_KEYS.contains(&key) {
					println!("{}={}", key, value.to_lowercase())
				} else {
					println!("{}={}", key, value);
				}
			}
		}
		Err(e) => eprintln!("{}", e)
	}
}

fn main() {
	let args = Args::parse();
	let legacy_ver = VersionReq::parse("<0.9.0").unwrap();
	if args.version {
		println!("EditorConfig (ec4rs-parse {}) Version {}", env!("CARGO_PKG_VERSION"), ec4rs::version::STRING);
	} else if args.files.len() == 1 {
		print_config(
			args.files.first().unwrap(),
			args.filename.as_ref(),
			legacy_ver.matches(&args.ec_version)
		);
	} else {
		for path in args.files {
			println!("[{}]", path.to_string_lossy());
			print_config(
				&path,
				args.filename.as_ref(),
				legacy_ver.matches(&args.ec_version)
			);
		}
	}
}
