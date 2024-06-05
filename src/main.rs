use clap::Parser;
use walker::Walker;

pub mod walker;

/// 🪄 Makefile generator
#[derive(Parser)]
struct Cli {
    /// Source code root, usually `src`
    path: String,

    /// Output directory
    #[arg(short, long, default_value_t = String::from("./target"))]
    output_path: String,

    /// Directories to ignore
    #[arg(short, long)]
    ignore_dirs: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let walker = Walker::new(cli.path, cli.ignore_dirs);
    let files = walker.walk();
    for file in files {
        println!("{}", file);
    }
}
