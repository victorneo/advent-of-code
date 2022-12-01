use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;


fn part_1() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let mut largest = 0;
    let mut current = 0;

    let lines : Vec<String> = f.lines().map(|x| x.unwrap()).collect();
    let len = lines.len();

    for (i, line) in lines.iter().enumerate() {
        if !line.is_empty() {
            current += line.parse::<u32>().unwrap();
        }

        if line.is_empty() || i == len-1 {
            if current > largest {
                largest = current;
            }
            current = 0;
        }
    }
    println!("Largest is {}", largest);

    Ok(())
}


fn part_2() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let mut largest1 = 0;
    let mut largest2 = 0;
    let mut largest3 = 0;
    let mut current = 0;

    let lines : Vec<String> = f.lines().map(|x| x.unwrap()).collect();
    let len = lines.len();

    for (i, line) in lines.iter().enumerate() {
        if !line.is_empty() {
            current += line.parse::<u32>().unwrap();
        }
        if line.is_empty() || i == len-1 {
            if current > largest1 {
                largest3 = largest2;
                largest2 = largest1;
                largest1 = current;
            }
            else if current > largest2 {
                largest3 = largest2;
                largest2 = current;
            }
            else if current > largest3 {
                largest3 = current;
            }
            current = 0;
        }
    }
    println!("Total is {}", largest1 + largest2 + largest3);

    Ok(())
}


fn main() -> io::Result<()> {
    part_1();
    part_2()
}
