use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[arg(short, long)]
    pattern: String,
    /// The path to the file to read
    #[arg(short, long)]
    file: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("pattern: {:?}, file: {:?}", args.pattern, args.file);
}
