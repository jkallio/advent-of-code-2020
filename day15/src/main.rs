fn main() {
    let mut history: Vec<i32> = vec![14, 1, 17, 0, 3, 20];
    for turn in history.len() + 1..=2020 {
        let mut age: i32 = 0;
        if let Some(test) = history.last() {
            for (i, v) in history.iter().rev().enumerate() {
                if i > 0 && *v == *test {
                    age = i as i32;
                    break;
                }
            }
        }
        println!("#{} = {}", turn, age);
        history.push(age);
    }
}
