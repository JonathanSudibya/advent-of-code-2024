use std::env;
use common;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let part = args[2].clone();

    let mut str_input = "".to_string();

    if let Ok(lines) = common::read_lines(filename) {
        for line in lines.flatten() {
            str_input.push_str(line.as_str());
        }
    }

    if part == "1".to_string() {
        part_one(str_input);
    } else if part == "2".to_string() {
        part_two(str_input);
    }
    
}

fn part_one(input: String) {
    let mut sum = 0;
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    for (_, [matches, op1, op2] ) in re.captures_iter(&input).map(|x| x.extract()) {
        println!("{} {} {}", matches, op1, op2);
        let int_op1: i32 = op1.to_string().parse().unwrap();
        let int_op2: i32 = op2.to_string().parse().unwrap();
        sum += int_op1 * int_op2;
    }
    println!("{}", sum);
}

fn part_two(input: String) {
    let mut sum = 0;
    let mut enabled = true;
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(()()\))|(don't\(()()\))").unwrap();
    for (_, [matches, op1, op2] ) in re.captures_iter(&input).map(|caps| caps.extract()) {
        println!("{} {} {}", matches, op1, op2);
        if matches == "don't()" {
            enabled = false
        } else if matches == "do()" {
            enabled = true
        } else if enabled {
            let int_op1: i32 = op1.to_string().parse().unwrap();
            let int_op2: i32 = op2.to_string().parse().unwrap();
            sum += int_op1 * int_op2;
        }
    }
    println!("{}", sum);
}