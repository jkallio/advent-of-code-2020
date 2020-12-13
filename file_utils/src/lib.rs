//! Crate for common utility functions for file reading
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

/// Reads given file line by line into a vector of Strings.
///
/// # Examples
/// ```
/// let file_path = "test_input_i32.txt";
/// if let Ok(vec) = file_utils::read_to_string_vec(&file_path) {
///     assert_eq!(vec.is_empty(), false);  // Valid Vec<String>
/// }
/// else {
///     assert!(false); // Handler std::io::Error
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

/// Reads given file line by line into a vector of i64-bit integers.
///
/// # Examples
/// ```
/// let file_path = "test_input_i32.txt";
/// if let Ok(vec) = file_utils::read_to_i64_vec(&file_path) {
///     assert_eq!(vec.is_empty(), false); // Valid Vec<i64>
/// }
/// else {
///     assert!(false); // Handler std::io::Error
/// }
/// ```
#[allow(dead_code)]
pub fn read_to_i64_vec(path: &str) -> Result<Vec<i64>, std::io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut vec = Vec::new();
    for line in br.lines() {
        let line = line?;
        let n: Result<i64, std::io::Error> = line
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e));
        vec.push(n?);
    }
    Ok(vec)
}

/// Reads given file line by line into a table of chars.
/// 
/// # Examples
/// ```
/// let file_path = "lorem_ipsum.txt";
/// if let Ok(vec) = file_utils::read_to_char_table(&file_path) {
///     assert_eq!(vec.is_empty(), false); // Valid Vec<Vec<char>>
/// }
/// else {
///     assert!(false); // Handler std::io::Error
/// }
/// ```
#[allow(dead_code)]
pub fn read_to_char_table(path: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut vec:Vec<Vec<char>> = Vec::new();
    for line in br.lines() {
        let line = line?;
        let mut cvec:Vec<char> = Vec::new();
        for c in line.chars() {
            cvec.push(c);
        }
        vec.push(cvec);
    }    
    Ok(vec)
}