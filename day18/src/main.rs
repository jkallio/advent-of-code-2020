use std::fs::File;
use std::io::{ BufReader, BufRead };
use regex::Regex;
//use regex::Matches;

fn solve(s:&str) -> String {
    let mut solved = String::from(s);
    println!("{}", solved);
    solved.retain(|c| c != '(' && c != ')');

    let parts = solved.split(' ');

    for p in parts {
        println!("{}", p);
    }

    //let re = Regex::new(r"\*|\+").unwrap();
    

    println!("{}", solved);

    solved
}

fn simplify_parentheses(s: &mut String) -> bool {
    
    let searh_str = s.clone();
    let re = Regex::new(r"\(([0-9](\s)*(\*|\+)(\s)*[0-9](\s)*((\*|\+)(\s)*[0-9]*(\s)*)*\))").unwrap();
    if let Some(m) = re.find(&searh_str) {
        let solved = solve(&s[m.start()..m.end()]);
        s.replace_range(m.start()..m.end(), &solved);
        return simplify_parentheses(s);
    }
    false
}

fn main() {
    let input = "input.txt";
    let file = File::open(input).unwrap();
    let br = BufReader::new(file);
        
    //let results = vec![];
    for line in br.lines() {
        let mut line = line.unwrap();
        println!("{}", line);
        //line.retain(|c| !c.is_whitespace());

        simplify_parentheses(&mut line);
    }
}
