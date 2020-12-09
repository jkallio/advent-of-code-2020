use file_reader;

fn main() {
    let input = "input.txt";
    let vec = file_reader::read_to_i64_vec(input);
    match vec {
        Ok(vec) => {
            for line in vec {
                println!("{}", line);
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    println!("End");
}
