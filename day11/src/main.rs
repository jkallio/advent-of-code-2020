fn main() {
    let input_file = "input.txt";
    let input = file_utils::read_to_string_vec(input_file).unwrap();
    for line in input {
        println!("{}", line);
    }
}