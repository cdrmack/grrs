use std::io::BufRead;

use clap::Parser;
use anyhow::{Context, Result};

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
    let file = std::fs::File::open(&args.path).with_context(|| format!("could not open file `{}`:", &args.path.display()))?;
    let reader = std::io::BufReader::new(file);

    for line_result in reader.lines() {
	let line = line_result.with_context(|| format!("could not read line"))?;
	if line.contains(&args.pattern) {
	    println!("{}", line);
	}
    }

    Ok(())
}
