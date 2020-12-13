use regex::Regex;

// Returns the earliest timestamp for departure with a list of bus ids
// Each bus also contains the delta time from t=0
fn parse_bus_schedule(input: &[String]) -> (i32, Vec<(i32, i32)>) {
    if input.len() == 2 {
        let mut departure: i32 = 0;
        let mut bus_list = Vec::new();

        // 1st line contains the earliest timestamp for departure
        if let Ok(i) = input[0].parse() {
            departure = i;
        }

        // Second line consists of bus id (also doubles as round-trip time) 
        let mut t = 0;
        for x in input[1].split(',') {
            let re = Regex::new(r"([0-9])+").unwrap();
            if re.is_match(x) {
                if let Ok(i) = x.parse() {
                    bus_list.push((i, t));
                }
            }
            t += 1;
        }
        return (departure, bus_list);
    }
    panic!("Invalid input");
}

// Returns the ID of the earliest bus you can take multiplied with the number of
// minutes you'll need to wait for the bus.
fn get_answer_for_part1(info: (i32, Vec<(i32, i32)>)) -> i32 {
    let departure = info.0;
    let mut closest_to_departure = (0, i32::MAX); // bus_id & delta from departure time

    for bus in info.1 {
        let modulo = departure % bus.0;
        let diff = (modulo - bus.0).abs();
        if diff < closest_to_departure.1 {
            closest_to_departure.1 = diff;
            closest_to_departure.0 = bus.0;
        }
    }
    closest_to_departure.0 * closest_to_departure.1
}


fn main() {
    if let Ok(input) = file_utils::read_to_string_vec("test_input.txt") {
        let info = parse_bus_schedule(&input);
        println!("Answer for part-1 is {}", get_answer_for_part1(info));        
    }
}
