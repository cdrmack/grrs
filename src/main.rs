struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("missing pattern to search for");
    let path = std::env::args().nth(2).expect("missing path");

    let args = Cli {
	pattern: pattern,
	path: std::path::PathBuf::from(path),
    };

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
