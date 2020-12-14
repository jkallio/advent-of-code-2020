use std::cmp::min;
use regex::Regex;
use std::cmp::Reverse;

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
    }

    if let Ok(input) = file_utils::read_to_string_vec("test_input1.txt") {
        let info = parse_bus_schedule(&input);
        
        // Part-1
        let part1 = get_answer_for_part1(&info);
        println!("Answer for part-1 is {}", part1);

        // Part-2
        let mut buses = Vec::new();
        for i in 0..info.1.len() {
            let bus = info.1[i];
            buses.push(Bus { 
                id:bus.0 as i64,
                t:bus.1 as i64,
            });
        }
        //buses.sort_by_key(|k| Reverse(k.id));
        //buses.sort_by_key(|k| k.id);

        let t0_bus_id = buses[0].id;
        //let t0_bus_t = buses[0].t;

        let mut biggest_match = 0;
        for i in 0..i64::MAX {

            let mut matches = 1;
            for j in 1..buses.len() {
                let bus = &buses[j];
                let result:f64 = ((t0_bus_id * i) as f64)/(bus.id as f64) + (bus.t as f64) / (bus.id as f64);
                if result.fract() == 0.0 {
                    matches += 1;
                }
                else { break; }
            }
            if matches > biggest_match {
                biggest_match = matches;
                println!("Iteration #{}; Biggest match={} (#{})", i, biggest_match, i);
            }
            if matches == buses.len() {
                println!("Result = {}", t0_bus_id * i);
                break;
            }
        }


        /*
        let mut target_t0 = 100000000000000;//0;
        let mut biggest_match = 1;
        loop {
            let mut matches = 0;
            for bus in buses.iter_mut() {
                let target = target_t0 + bus.t;
                while bus.timestamp < target {
                    bus.timestamp += bus.id;
                }
                if bus.timestamp > target {
                    target_t0 = bus.timestamp- bus.t;
                    break;
                }
                else if bus.timestamp == target {
                    matches += 1;
                    if matches > biggest_match {
                        biggest_match = matches;
                        println!("Matches = {}; target_t0 = {}", matches, target_t0);
                    }
                }
            }  
            if matches == buses.len() {
                println!("FOUND!");
                break;
            }
        }
        println!("{}", target_t0);
        */
    }
}
