use std::*;
use std::collections::HashMap;
use common;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let part = args[2].clone();

    let mut array1: Vec<i32> = Vec::new(); 
    let mut array2: Vec<i32> = Vec::new();

    if let Ok(lines) = common::read_lines(filename) {
        for line in lines.flatten() {
            let line_split = line.split("   ");
            let mut i = 0;
            for part in line_split {
                let parsed_int: i32 = part.parse().unwrap();
                if i == 0 {
                    array1.push(parsed_int);
                    i += 1;
                } else {
                    array2.push(parsed_int);
                }
            }
        }
    }

    if part == "1".to_string() {
        part_one(&mut array1, &mut array2);
    } else if part == "2".to_string() {
        part_two(&mut array1, &mut array2);
    }
    
}


fn part_one(array1: &mut Vec<i32>, array2: &mut Vec<i32>) {
    array1.sort();
    array2.sort();

    let mut index = 0;
    let array_size = array1.len();
    let mut diff = 0;
    while index < array_size {
        let num1 = array1[index];
        let num2 = array2[index];

        if num1 < num2 {
            diff += num2 - num1 
        } else {
            diff += num1 - num2
        }
         
        index += 1;
    };    

    println!("{}", diff);
}

fn part_two(array1: &mut Vec<i32>, array2: &mut Vec<i32>) {
    // first we need to scan the second array and map it how often it occurs
    let mut array2_occurance = HashMap::new();

    for num in array2 {
        array2_occurance.entry(*num).and_modify(|occ| *occ += 1).or_insert(1);
    }

    let mut similarity = 0;
    for num in array1 {
        similarity += match array2_occurance.get(num) {
            Some(x) => *x * *num,
            None => 0
        };
    }

    println!("{}", similarity);
}