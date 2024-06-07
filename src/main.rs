use clap::{Parser, ValueEnum};
use maker::Maker;
use walker::Walker;

pub mod lang;
pub mod maker;
pub mod walker;

/// Supported programming languages.
#[derive(ValueEnum, Clone, Debug)]
enum Language {
    Cpp,
    C,
}

/// 🪄 Makefile generator
#[derive(Parser)]
struct Cli {
    /// The paths to search for files. Common paths are `src` and `include`
    #[arg(num_args(1..), required(true))]
    paths: Vec<String>,

    /// Output directory for build files
    #[arg(short, long, default_value_t = String::from("target"))]
    output_path: String,

    /// Directories to ignore
    #[arg(short, long, num_args(0..))]
    exclude_dirs: Vec<String>,

    /// Programming language
    #[arg(short, long, value_enum, default_value_t = Language::Cpp)]
    lang: Language,
}

fn main() {
    let cli = Cli::parse();
    let walker = Walker::new(cli.paths, cli.exclude_dirs);
    let files = walker.walk();
    // Can this be abstracted away?
    let mut maker = match cli.lang {
        Language::Cpp | Language::C => lang::cpp::cpp::CppMaker::new(files, cli.output_path),
    };
    maker.build();
    println!("{}", maker.dump());
}
