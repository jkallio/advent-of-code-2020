//! Crate for common utility functions for file reading
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};


/// Reads given file line by line into a vector of Strings.
/// 
/// # Examples
/// ```
/// let file_path = "my_file.txt";
/// if let Ok(vec) = parse_input_file(&file_path) {
///     // Valid Vec<String>
/// }
/// else {
///     // Handle std::io::Error
/// }
/// ```
#[allow(dead_code)]
pub fn read_to_string_vec(path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut vec = Vec::new();
    for line in br.lines() {
        vec.push(line?);
    }
    Ok(vec)
}

#[allow(dead_code)]
pub fn read_to_i32_vec(path: &str) -> Result<Vec<i32>, std::io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut vec = Vec::new();
    for line in br.lines() {
        let line = line?;
        let n:Result<i32, std::io::Error>= line.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e));
        vec.push(n?);
    }
    Ok(vec)
}
/* TODO: Write unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/