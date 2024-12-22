use std::env;
use common;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let part = args[2].clone();

    let mut str_input: Vec<String> = Vec::new();

    if let Ok(lines) = common::read_lines(filename) {
        for line in lines.flatten() {
            str_input.push(line);
        }
    }

    if part == "1".to_string() {
        part_one(str_input);
    } else if part == "2".to_string() {
        part_two(str_input);
    } 
}

fn part_one(input: Vec<String>) {
    let target = String::from("XMAS");
    let mut j = 0;
    let mut sum = 0;
    while j < input.len() {
        let mut i = 0;
        while i < input[j].len() {
            let cur_char = input[j].chars().nth(i).unwrap();
            if cur_char == 'X' {
                sum += find_string(&target, &input, 0, i, j, 0, 0, input[j].len(), input.len());
            }
            i +=1;
        }
        j += 1;
    }
    println!("{}", sum)
}

fn part_two(input: Vec<String>) {
    // find A then check cross for M and S.
    let mut j = 1;
    let mut sum = 0;
    while j < input.len()-1 {
        let mut i = 1;
        while i < input[j].len()-1 {
            let cur_char = input[j].chars().nth(i).unwrap();
            if cur_char == 'A' {
                sum += find_cross(i, j, &input)
            }
            i +=1;
        }
        j += 1;
    }
    println!("{}", sum)
}

fn find_string(target: &String, input: &Vec<String>, current_index: usize, x: usize, y: usize, dir_x: i32, dir_y: i32, max_x: usize, max_y: usize) -> i32 {
    if current_index == target.len() {
        return 1
    }
    
    if x >= max_x || y >= max_y {
        return 0
    }

    let mut sum = 0;

    if dir_x == 0 && dir_y == 0 {
        // no direction set
        let mut i = -1;
        while i <= 1 {
            let mut j = -1;
            while j <= 1 {
                if i == 0 && j == 0 {
                    // skip to prevent infinite looping
                    j += 1;
                    continue;
                }
                sum += find_string(target, input, current_index, x, y, i, j, max_x, max_y);
                j += 1;
            }
            i += 1;
        }
    } else {
        let cur_char = target.chars().nth(current_index).unwrap();
        let example_char = input[y].chars().nth(x).unwrap();
        if cur_char == example_char {
            let mut new_x = x;
            let mut new_y = y;
            if dir_x > 0 {
                new_x += 1;
            } else if dir_x < 0 {
                if x == 0 {
                    new_x = max_x;
                } else {
                    new_x -= 1;
                }
            }
            if dir_y > 0 {
                new_y += 1;
            } else if dir_y < 0 {
                if y == 0 {
                    new_y = max_y;
                } else {
                    new_y -= 1;
                }
            }
            sum += find_string(target,input, current_index+1, new_x, new_y, dir_x, dir_y, max_x, max_y);
        }
    }

    sum
}

// find cross of the target string. e.g. M and S
// if M is found, check for S in diagonal direction. applicable in other way around
// if both are found, increment sum by 1
// if not found, increment sum by 0
fn find_cross(x: usize, y: usize, input: &Vec<String>) -> i32 {
    let mut strings_vec = [input[y-1].chars().nth(x-1).unwrap(),input[y+1].chars().nth(x+1).unwrap(), input[y-1].chars().nth(x+1).unwrap(), input[y+1].chars().nth(x-1).unwrap()];
    // sort the string pairs to make it easier to check
    if strings_vec[0] < strings_vec[1] {
        strings_vec.swap(0,1);
    }
    if strings_vec[2] < strings_vec[3] {
        strings_vec.swap(2,3);
    }

    if (strings_vec[0] == strings_vec[2] && strings_vec[0] == 'S' ) && (strings_vec[1] == strings_vec[3] && strings_vec[1] == 'M') {
        return 1
    } else {
        return 0
    }
}