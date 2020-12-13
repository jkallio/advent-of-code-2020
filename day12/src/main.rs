fn main() {
    let input = file_utils::read_to_string_vec("input.txt");
    for i in input {
        println!("{:?}", i);
    }
}
