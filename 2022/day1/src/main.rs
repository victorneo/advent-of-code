use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_lines(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    return f.lines().map(|x| x.unwrap()).collect();
}

fn part_1() {
    let mut largest = 0;
    let mut current = 0;

    let lines: Vec<String> = read_lines("input.txt");
    let len = lines.len();

    for (i, line) in lines.iter().enumerate() {
        if !line.is_empty() {
            current += line.parse::<u32>().unwrap();
        }

        if line.is_empty() || i == len - 1 {
            if current > largest {
                largest = current;
            }
            current = 0;
        }
    }
    println!("Largest is {}", largest);
}

fn part_2() {
    let mut largest1 = 0;
    let mut largest2 = 0;
    let mut largest3 = 0;
    let mut current = 0;

    let lines: Vec<String> = read_lines("input.txt");
    let len = lines.len();

    for (i, line) in lines.iter().enumerate() {
        if !line.is_empty() {
            current += line.parse::<u32>().unwrap();
        }
        if line.is_empty() || i == len - 1 {
            if current > largest1 {
                largest3 = largest2;
                largest2 = largest1;
                largest1 = current;
            } else if current > largest2 {
                largest3 = largest2;
                largest2 = current;
            } else if current > largest3 {
                largest3 = current;
            }
            current = 0;
        }
    }
    println!("Total is {}", largest1 + largest2 + largest3);
}

fn main() {
    part_1();
    part_2();
}
