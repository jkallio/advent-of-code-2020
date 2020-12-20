use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Solves the equations inside parentheses and returns the given equation in form where all parentheses have been resolved
// Call itself recursively (each recursion solves one equation inside parentheses)
fn recursive_simplify_parentheses(s: &str) -> String {
    let re = Regex::new(r"\(([0-9]+(\s)*(\*|\+)(\s)*[0-9]+(\s)*((\*|\+)(\s)*[0-9]+(\s)*)*\))").unwrap();
    let mut equation = String::from(s);
    if let Some(m) = re.find(&s) {
        let solved = solve_equation_with_precedence(&s[m.start() + 1..m.end() - 1]);
        equation.replace_range(m.start()..m.end(), &solved.to_string());
        equation = recursive_simplify_parentheses(&equation);
    }
    equation
}

// As '+' is to be counted before '*' this utility function adds parentheses around the sum operation in the equation
fn recursive_add_parentheses(s: &str) -> String {
    let re = Regex::new(r"[0-9]+ \+ [0-9]+").unwrap();
    let mut equation = String::from(s);
    if let Some(m) = re.find(&s) {
        let solved = solve_equation_with_precedence(&s[m.start()..m.end()]);
        equation.replace_range(m.start()..m.end(), &solved.to_string());
        equation = recursive_add_parentheses(&equation);
    }
    equation
}

// Solves the equation by first simplifying parentheses and then adding/multiplying values from left to right
// Note that in this problem '+' and '*' have the swapped precedence ('+' is counted before '*')
fn solve_equation_with_precedence(s: &str) -> i64 {
    enum Sign {
        MULTIPLICATION,
        ADDITION,
    }
    
    let mut equation = String::from(s);
    
    // Solve the parentheses first
    if equation.contains('(') {
        equation = recursive_simplify_parentheses(&equation);
    }
    // Add parentheses around sum operations so that they will be calculated first
    if equation.contains('+') && equation.contains('*') {
        equation = recursive_add_parentheses(&equation);
    }
    
    // Solve the simplified equation where all parentheses have been removed
    let mut sign = Sign::ADDITION;
    let mut result: i64 = 0;
    for p in equation.split(' ') {
        match p {
            "*" => {
                sign = Sign::MULTIPLICATION;
            }
            "+" => {
                sign = Sign::ADDITION;
            }
            _ => {
                let operand = p.parse::<i64>().unwrap();

                match sign {
                    Sign::ADDITION => {
                        result += operand;
                    }
                    Sign::MULTIPLICATION => {
                        result *= operand;
                    }
                }
            }
        }
    }
    println!("   -> {} = {}", equation, result);
    result
}

// Solves all the equations from given input file and sums the results together
fn main() {
    let input = "input.txt";
    let file = File::open(input).unwrap();
    let br = BufReader::new(file);

    let mut results = vec![];
    for line in br.lines() {
        let line = line.unwrap();
        println!("{}", line);
        let result = solve_equation_with_precedence(&line);
        results.push(result);
        println!("----------------------------------");
    }

    let mut answer = 0;
    for val in results {
        answer += val;
    }
    println!("Final answer is {}", answer);
}
