use regex::Regex;

fn parse_bus_schedule(input: &[String]) -> (i32, Vec<i32>) {
    if input.len() == 2 {
        let mut schedule: i32 = 0;
        let mut buses = Vec::new();
        if let Ok(i) = input[0].parse() {
            schedule = i;
        }
        for x in input[1].split(',') {
            let re = Regex::new(r"([0-9])+").unwrap();
            if re.is_match(x) {
                if let Ok(i) = x.parse() {
                    buses.push(i);
                }
            }
        }
        return (schedule, buses);
    }
    panic!("Invalid input");
}

fn main() {
    if let Ok(input) = file_utils::read_to_string_vec("input.txt") {
        let info = parse_bus_schedule(&input);
        println!("{}", info.0);
        let mut smallest = (0, i32::MAX);
        for bus in info.1 {
            let modulo = info.0 % bus;
            let diff = (modulo - bus).abs();
            if diff < smallest.1 {
                smallest.1 = diff;
                smallest.0 = bus;
            }
        }
        println!("Answer for part1 = {}", smallest.0 * smallest.1);
    }
}
