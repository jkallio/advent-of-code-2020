fn main() {
    let input = "input.txt"; 
    let mut vec = file_reader::read_to_i64_vec(input).unwrap();
    vec.sort();
    let mut diff_1 = 1; // Starting from 1 because first outlet is 0 and first adapter is 1
    let mut diff_3 = 1; // Starting from 3 because internal adapter is 3 higher than highest adapter in the bag
    for adapters in vec.windows(2) {
        let diff = adapters[1] - adapters[0];
        match diff {
            1 => { diff_1 += 1; }
            3 => { diff_3 += 1; }
            _ => { panic!(""); }
        }
        //println!("{} --> ({}-{})", adapters[1], diff_1, diff_3);
    }
    println!("{}", diff_1 * diff_3);
}
