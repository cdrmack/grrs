use clap::Parser;
use anyhow::{Context, Result, anyhow};

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if args.pattern.is_empty() {
	return Err(anyhow!("pattern cannot be empty"));
    }

    let file = std::fs::File::open(&args.path).with_context(|| format!("could not open file `{}`:", &args.path.display()))?;
    let reader = std::io::BufReader::new(file);

    grrs::find_matches(&args.pattern, reader, &mut std::io::stdout())
}
