use clap::Parser;
use walker::Walker;

pub mod lang;
pub mod maker;
pub mod walker;

/// 🪄 Makefile generator
#[derive(Parser)]
struct Cli {
    /// The paths to search for files. If none are provided, the current directory is used.
    #[arg(num_args(0..))]
    paths: Vec<String>,

    /// Output directory for build files
    #[arg(short, long, default_value_t = String::from("./target"))]
    output_path: String,

    /// Directories to ignore
    #[arg(short, long, num_args(0..))]
    exclude_dirs: Vec<String>,
}

fn main() {
    let mut cli = Cli::parse();
    if cli.paths.is_empty() {
        println!("No paths provided. Using current directory.");
        cli.paths.push(String::from("."));
    }

    let walker = Walker::new(cli.paths, cli.exclude_dirs);
    let files = walker.walk();
    for file in files {
        println!("{}", file);
    }
}
