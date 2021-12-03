use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

/// Read lines of a file
///
/// # Errors
///
/// Will return an `Err` if `filename` doesn't exist or `filename` can't be read.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<str>,
{
    let mut filepath: PathBuf = std::env::current_dir()?;
    filepath.push("data");
    filepath.push(filename.as_ref());
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
