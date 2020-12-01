use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

// Split file into vector of integers.
// Returns vector / std::io::Error
fn read(path: &str) -> Result<Vec<i32>, std::io::Error> {

    // Open the file
    let file = File::open(path)?;

    // Wrap file into generic buffered reader
    let bufr = BufReader::new(file);

    // Create new mutable vector
    let mut v = Vec::new();

    // Iterate the reader line by line
    for line in bufr.lines() {
        // IO operations generally return error
        let line = line?;
        let n = line
            .trim() // trim "whitespaces"
            .parse() // parse i32 from string
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?; // parse() can return error --> Convert to std::io::Error
        v.push(n); // push acquired integer to the vector
    }
    Ok(v) // Return vector
}

fn main() -> Result<(), Error> {
    let filename = "input.txt";
    let vec = read(filename);
    if let Ok(vec) = vec {

        // Iterate through the vector, and find the two addends of sum 2020
        for x in 0..vec.len() {
            for y in 0..vec.len() {
                if vec[x] + vec[y] == 2020 {
                    println!("{}: {} = {}", vec[x], vec[y], vec[x] * vec[y]);
                }
            }
        }

        // Iterate through the vector, and find three addends of sum 2020
        for x in 0..vec.len() {
            for y in 0..vec.len() {
                for z in 0..vec.len() {
                    if vec[x] + vec[y] + vec[z] == 2020 {
                        println!("{} + {} + {} = {}", vec[x], vec[y], vec[z], vec[x] + vec[y] + vec[z]);
                        println!("{} * {} * {} = {}", vec[x], vec[y], vec[z], vec[x] * vec[y] * vec[z]);
                    }
                }
            }
        }
    }
    Ok(())
}

/*
NOTES:
BufReader<R> can improve the speed of programs that make small and repeated read calls to the same
file or network socket. It does not help when reading very large amounts at once, or reading just 
one or a few times. It also provides no advantage when reading from a source that is already in
memory, like a Vec<u8>.
 */