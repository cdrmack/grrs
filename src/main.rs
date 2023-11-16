use std::io::BufRead;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let file = std::fs::File::open(&args.file)?;
    let reader = std::io::BufReader::new(file);

    for line_result in reader.lines() {
	let line = line_result?;
	if line.contains(&args.pattern) {
	    println!("{}", line);
	}
    }

    Ok(())
}
