use std::collections::HashMap;

fn main() {
    // Initialize the game
    let input: Vec<i32> = vec![14, 1, 17, 0, 3, 20];
    let mut history = HashMap::<i32, i32>::new();
    let mut last_spoken = 0;
    for (i, n) in input.iter().enumerate() {
        if n == input.last().unwrap() {
            last_spoken = *n;
        } else {
            history.insert(*n, 1 + (i as i32));
        }
    }

    // Play the game
    for turn in (input.len() + 1)..=30000000 {
        let mut age: i32 = 0;
        if let Some(value) = history.get(&last_spoken) {
            age = turn as i32 - *value - 1;
        }
        history.insert(last_spoken, turn as i32 - 1);
        last_spoken = age;
    }
    println!("Last spoken number is {}", last_spoken);
}
