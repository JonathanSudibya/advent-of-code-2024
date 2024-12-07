use std::*;
use common;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let part = args[2].clone();

    let mut reports: Vec<Vec<i32>> = Vec::new(); 

    if let Ok(lines) = common::read_lines(filename) {
        for line in lines.flatten() {
            let line_split = line.split(" ");
            let mut report = Vec::new();
            for part in line_split {
                let parsed_int: i32 = part.parse().unwrap();
                report.push(parsed_int);
            }
            reports.push(report);
        }
    }

    if part == "1".to_string() {
        part_one(&mut reports);
    } else if part == "2".to_string() {
        part_two(&mut reports);
    }
    
}


fn part_one(reports: &mut Vec<Vec<i32>>) {
    let mut safe = 0;
    for report in reports {
        let s = is_safe(&report);
        if s {
            safe += 1;
        }
    }
    println!("{}", safe);
}

fn part_two(reports: &mut Vec<Vec<i32>>) {
    let mut safe = 0;
    for report in reports {
        let s = is_safe_with_dampener(report);
        if s {
            safe += 1;
        }
    }
    println!("{}", safe);
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    let mut index = 0;
    while index < levels.len()-1 {
        let diff = levels[index+1] - levels[index];
        if diff < -3 || diff > 3 || diff == 0 {
            return false
        }
        if levels[index+1] < levels[index] {
            increasing = false;
        }
        if levels[index+1] > levels[index] {
            decreasing = false;
        }
        index += 1;
    }

    increasing || decreasing
}

fn is_safe_with_dampener(levels: &mut Vec<i32>) -> bool {
    if is_safe(levels) {
        return true
    }
    let mut i = 0;
    while i < levels.len() {
        let mut reduced = levels.clone();
        reduced.remove(i);
        if is_safe(&reduced) {
            return true
        }
        i += 1;
    }
    false
}
