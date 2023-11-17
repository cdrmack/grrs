use std::io::BufRead;

use anyhow::{Context, Result};

pub fn find_matches(pattern: &str, reader: std::io::BufReader<std::fs::File>, mut writer: impl std::io::Write) -> Result<()> {
    for line_result in reader.lines() {
    let line = line_result.with_context(|| format!("could not read line"))?;
        if line.contains(pattern) {
	    writeln!(writer, "{}", line)?;
	}
    }
    Ok(())
}
