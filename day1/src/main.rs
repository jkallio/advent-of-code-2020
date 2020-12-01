use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

// Split file into vector of integers.
// Returns vector / std::io::Error
fn split_file(path: &str) -> Result<Vec<i64>, std::io::Error> {

    // Open the file
    let file = File::open(path)?;

    // Wrap file into generic buffered reader
    let bufr = BufReader::new(file);

    // NOTE! BufReader<R> can improve the speed of programs that make small and repeated read calls 
    // to the same file or network socket. It does not help when reading very large amounts at once, 
    // or reading just one or a few times. It also provides no advantage when reading from a source 
    // that is already in memory, like a Vec<u8>.

    // Create new mutable vector
    let mut v = Vec::new();

    // Iterate the reader line by line
    for line in bufr.lines() {
        // IO operations generally return error
        let line = line?;
        let n = line
            .trim() // trim "whitespaces"
            .parse() // parse i64 from string
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?; // parse() can return error --> Convert to std::io::Error
        v.push(n); // push acquired integer to the vector
    }
    Ok(v) // Return vector
}

// Main function
fn main() -> Result<(), Error> {
    let filename = "input.txt";
    let vec = split_file(filename);
    if let Ok(vec) = vec {

        let addends = Vec::<i64>::new();
        recursive_find_addends(&vec, &addends, 2020, 3); 
    }
    return Ok(())
}

// Return the sum of vector elements
fn sum_of_vector_elements(vec: &Vec<i64>) -> i64 {
    let mut sum:i64 = 0;
    for a in vec {
        sum += a;
    }
    return sum;
}

// Find addends that has total sum of given sum_target
// Calls recursively 
fn recursive_find_addends(vec: &Vec<i64>, addends:&Vec<i64>, sum_target:i64, iter:i64) -> bool {

    if sum_of_vector_elements(addends) == sum_target {
        let mut product = 1;
        for a in addends {
            product *= a;
            print!("{} ", a);
        }
        println!(" => {}", product);
        return true;
    }
    else if iter > 0 {
        for x in 0..vec.len() {
            let mut a:Vec<i64> = addends.clone();
            a.push(vec[x]);
            if recursive_find_addends(vec, &a, sum_target, iter-1) {
                return true;
            }
        }
    }
    return false;
}
