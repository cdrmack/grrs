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

fn main() {
    let args = Cli::parse();
    let file = std::fs::File::open(&args.file).expect("could not open file");
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
	match line {
	    Ok(text) => if text.contains(&args.pattern) {
		println!("{}", text);
	    },
	    Err(e) => panic!("could not read line: {:?}", e),
	}
    }
}
