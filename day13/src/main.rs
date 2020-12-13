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
fn get_answer_for_part1(info: &(i32, Vec<(i32, i32)>)) -> i32 {
    let departure = info.0;
    let mut closest_to_departure = (0, i32::MAX); // bus_id & delta from departure time

    for bus in &info.1 {
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
    struct Bus {
        id: i64,
        t: i64,
        timestamp:i64,
        addon:i64,        
    }

    if let Ok(input) = file_utils::read_to_string_vec("test_input1.txt") {
        let info = parse_bus_schedule(&input);
        
        // Part-1
        let part1 = get_answer_for_part1(&info);
        println!("Answer for part-1 is {}", part1);

        // Part-2
        let mut buses = Vec::new();
        for bus in info.1 {
            let bus = Bus { 
                id:bus.0 as i64,
                t:bus.1 as i64,
                timestamp:bus.0 as i64,
                addon:(bus.0 - bus.1) as i64,
            };
            buses.push(bus);
        }
        buses.sort_by_key(|k| k.id);

        let mut target_t0 = 0;
        let mut biggest_match = 1;
        loop {
            let mut matches = 0;
            for bus in buses.iter_mut() {
                let target = target_t0 + bus.t;
                while bus.timestamp < target {
                    bus.timestamp += bus.id;
                }
                if bus.timestamp > target {
                    target_t0 = bus.timestamp - bus.t;
                    break;
                }
                else if bus.timestamp == target {
                    matches += 1;
                    if matches > biggest_match {
                        biggest_match = matches;
                        bus.addon = target_t0;
                        println!("Matches = {}; target_t0 = {};", matches, target_t0);
                    }
                }
            }  
            if matches == buses.len() {
                println!("FOUND!");
                break;
            }
        }
        println!("{}", target_t0);
    }
}
