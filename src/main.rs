use std::io::BufRead;

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();
    let file = std::fs::File::open(&args.path).map_err(|err| CustomError(format!("Error reading path `{}`: {}", args.path.display(), err)))?;
    let reader = std::io::BufReader::new(file);

    for line_result in reader.lines() {
	let line = line_result.map_err(|err| CustomError(format!("Error reading line {}", err)))?;
	if line.contains(&args.pattern) {
	    println!("{}", line);
	}
    }

    Ok(())
}
