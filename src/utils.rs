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

/// Read a file line by line and try parse each line
///
/// # Errors
///
/// `IoError` if filename id not valid
///
/// # Panics
///
/// Panics if parsing is invalid
pub fn read_lines_to_t_iterator<P: AsRef<str>, T: std::str::FromStr>(
    filename: P,
) -> impl Iterator<Item = T> {
    read_lines(filename.as_ref())
        .unwrap()
        .map(Result::unwrap)
        .map(|l| l.trim().parse::<T>())
        .map(|r| r.ok().unwrap())
}

/// Read a file line by line and try parse each line
///
/// # Errors
///
/// `IoError` if filename id not valid
///
/// # Panics
///
/// Panics if parsing is invalid
pub fn read_lines_to_vec_t_<P: AsRef<str>, T: std::str::FromStr>(filename: P) -> Vec<T> {
    read_lines_to_t_iterator(filename.as_ref()).collect()
}

/// Read a file line by line and try parse each line
///
/// # Errors
///
/// `IoError` if filename id not valid
///
/// # Panics
///
/// Panics if parsing is invalid
pub fn read_single_string_to_t_vec<P: AsRef<str>, T: std::str::FromStr + Clone>(
    filename: P,
    pat: char,
) -> Vec<T> {
    let mut filepath: PathBuf = std::env::current_dir().unwrap();
    filepath.push("data");
    filepath.push(filename.as_ref());

    std::fs::read_to_string(filepath)
        .unwrap()
        .split(pat)
        .map(|s| s.trim().parse())
        .map(|s| s.ok().unwrap())
        .collect::<Vec<_>>()
}
