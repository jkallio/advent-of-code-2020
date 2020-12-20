use std::fs::File;
use std::io::{ BufReader, BufRead };
use regex::Regex;

fn solve(s:&str) -> i64 {
    enum Sign {
        MULTIPLICATION,
        ADDITION,
    }
    let equation = String::from(s);
    /*
    while equation.contains('(') {
        simplify_parentheses(&mut equation);
    }
    //equation.retain(|c| c != '(' && c != ')');
    */
    let mut sign = Sign::ADDITION;
    let mut result:i64 = 0;
    for p in equation.split(' ') {
        match p {
            "*" => { sign = Sign::MULTIPLICATION; }
            "+" => { sign = Sign::ADDITION; }
            _ => { 
                let operand = p.parse::<i64>().unwrap();

                match sign {
                    Sign::ADDITION => { result += operand; }
                    Sign::MULTIPLICATION => { result *= operand; }
                }
            }
        }
    }
    println!("{} = {}", equation, result);
    result
}

fn simplify_parentheses(s: &mut String) -> bool {
    
    let searh_str = s.clone();
    let re = Regex::new(r"\(([0-9]+(\s)*(\*|\+)(\s)*[0-9]+(\s)*((\*|\+)(\s)*[0-9]+(\s)*)*\))").unwrap();
    if let Some(m) = re.find(&searh_str) {
        let solved = solve(&s[m.start()+1..m.end()-1]);
        s.replace_range(m.start()..m.end(), &solved.to_string());
        return simplify_parentheses(s);
    }
    false
}

fn main() {
    let input = "input.txt";
    let file = File::open(input).unwrap();
    let br = BufReader::new(file);
        
    let mut results = vec![];
    for line in br.lines() {
        let mut line = line.unwrap();
        println!("{}", line);
        loop {
            if simplify_parentheses(&mut line) == false {
                let result:i64 = solve(&line);
                results.push(result);
                break;
            }
        }
        println!("----------------------------------");
    }

    let mut answer = 0;
    for val in results {
        answer += val;
    }
    println!("Final answer is {}", answer);
}
