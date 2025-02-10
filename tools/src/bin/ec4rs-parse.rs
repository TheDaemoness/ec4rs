use std::path::PathBuf;

use clap::Parser;
use semver::{Version, VersionReq};

#[derive(Parser)]
struct DisplayArgs {
    /// Prefix each line with the path to the file where the value originated
    #[clap(short = 'H', long)]
    with_filename: bool,
    /// Prefix each line with the line number where the value originated
    #[clap(short = 'n', long)]
    line_number: bool,
    /// Use the NUL byte as a field delimiter instead of ':'
    #[clap(short = '0', long)]
    null: bool,
}

#[derive(Parser)]
#[clap(disable_version_flag = true)]
struct Args {
    #[clap(flatten)]
    display: DisplayArgs,
    /// Override config filename
    #[clap(short)]
    filename: Option<PathBuf>,
    /// Mostly ignored by this implementation
    #[clap(default_value = ec4rs::version::STRING, short = 'b')]
    ec_version: Version,
    /// Print test-friendly version information
    #[clap(short, long)]
    version: bool,
    files: Vec<PathBuf>,
}

fn print_empty_prefix(display: &DisplayArgs) {
    if display.with_filename {
        print!("{}", if display.null { '\0' } else { ':' });
    }
    if display.line_number {
        print!("{}", if display.null { '\0' } else { ':' });
    }
}

fn print_config(
    path: &std::path::Path,
    filename: Option<&PathBuf>,
    legacy_fallbacks: bool,
    display: &DisplayArgs,
) {
    match ec4rs::properties_from_config_of(path, filename) {
        Ok(mut props) => {
            if legacy_fallbacks {
                props.use_fallbacks_legacy();
            } else {
                props.use_fallbacks();
            }
            for (key, value) in props.iter() {
                let mut lc_value: Option<ec4rs::rawvalue::RawValue> = None;
                let value_ref = if ec4rs::property::STANDARD_KEYS.contains(&key) {
                    lc_value.get_or_insert(value.to_lowercase())
                } else {
                    value
                };
                if let Some((path, line_no)) = value_ref.source() {
                    if display.with_filename {
                        print!(
                            "{}{}",
                            path.to_string_lossy(),
                            if display.null { '\0' } else { ':' }
                        );
                    }
                    if display.line_number {
                        print!("{}{}", line_no, if display.null { '\0' } else { ':' });
                    }
                } else {
                    print_empty_prefix(display);
                }
                println!("{}={}", key, value_ref)
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}

fn main() {
    let args = Args::parse();
    let legacy_ver = VersionReq::parse("<0.9.0").unwrap();
    if args.version {
        println!(
            "EditorConfig (ec4rs-parse {}) Version {}",
            env!("CARGO_PKG_VERSION"),
            ec4rs::version::STRING
        );
    } else if args.files.len() == 1 {
        print_config(
            args.files.first().unwrap(),
            args.filename.as_ref(),
            legacy_ver.matches(&args.ec_version),
            &args.display,
        );
    } else {
        for path in args.files {
            print_empty_prefix(&args.display);
            println!("[{}]", path.to_string_lossy());
            print_config(
                &path,
                args.filename.as_ref(),
                legacy_ver.matches(&args.ec_version),
                &args.display,
            );
        }
    }
}
