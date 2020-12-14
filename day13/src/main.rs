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
        for (t, x) in input[1].split(',').enumerate() {
            let re = Regex::new(r"([0-9])+").unwrap();
            if re.is_match(x) {
                if let Ok(i) = x.parse() {
                    bus_list.push((i, t as i32));
                }
            }
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

// Brute force through all possible indexes in i64 range. This solution is good enough
// for test inputs with small amount of buses, but won't solve the actual puzzle input
// as it would take too long to iterate through the entire range.
#[allow(dead_code)]
fn brute_force_part2(info: &(i32, Vec<(i32, i32)>)) -> i64 {
    let t0_bus_id: i64 = info.1[0].0 as i64;
    for i in 0..i64::MAX {
        let mut matches = 1;
        for j in 1..info.1.len() {
            let bus = &info.1[j];
            let result: f64 =
                ((t0_bus_id * i) as f64) / (bus.0 as f64) + (bus.1 as f64) / (bus.0 as f64);
            if result.fract() == 0.0 {
                matches += 1;
            } else {
                break;
            }
        }
        if matches == info.1.len() {
            return t0_bus_id * i;
        }
    }
    panic!("Run out of i64 range");
}

// Returns the answer by using the Chinese Remainder Theorem
fn chinese_remainder_theorem(info: &(i32, Vec<(i32, i32)>)) -> i64 {
    struct Bus {
        id: i64,
        remainder: i64,
        sum: i64,
    }

    // Parse input bus schedules into above struct format
    let mut tot_modulo: i64 = 1;
    let mut vec = Vec::new();
    for x in &info.1 {
        vec.push(Bus {
            id: x.0 as i64,
            remainder: x.1 as i64,
            sum: 1,
        });
        tot_modulo *= x.0 as i64; // Final result is the modulo of all modulos combined
    }

    // Multiply the sum for each bus with all the other buses modulos to get rid of them
    // for each single bus schedule
    for i in 0..vec.len() {
        for j in 0..vec.len() {
            if i != j {
                vec[i].sum *= vec[j].id;
            }
        }
    }

    // Find the correct remainder for each bus schedule
    // by iterating multipliers 1..i64.MAX
    let mut result: i64 = 0;
    for bus in &mut vec {
        // Find the target for remainder (diff from t=0)
        let mut remainder_target: i64 = 0;
        if bus.remainder > 0 {
            remainder_target = bus.id - (bus.remainder % bus.id);
        }

        // Find the correct remainder for each bus schedule
        for x in 1..i64::MAX {
            let test: i64 = bus.sum * x as i64;
            if test % bus.id == remainder_target {
                bus.sum = test;
                result += bus.sum;
                break;
            }
        }
    }

    // The final result is the smallest matching timestamp
    result % tot_modulo
}

fn main() {
    if let Ok(input) = file_utils::read_to_string_vec("input.txt") {
        let info = parse_bus_schedule(&input);

        // Part-1
        let part1 = get_answer_for_part1(&info);
        println!("Answer for part-1 is {}", part1);

        // Part-2
        //let part2: i64 = brute_force_part2(&info);
        //println!("Answer for part-2 is {}", part2);

        // 554865447501099
        let part2: i64 = chinese_remainder_theorem(&info);
        println!("Answer for part-2 is {}", part2);
    }
}
