use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_lines(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    return f.lines().map(|x| x.unwrap()).collect();
}

fn get_priority(c: char) -> u32 {
    let mut priority = c as u32;
    // a = 97, A = 65
    if priority > 96 {
        priority -= 96;
    }
    else {
        priority -= 38;
    }

    return priority; 
}


fn part_1() {
    let lines: Vec<String> = read_lines("input.txt");
    let mut total :u32 = 0;

    for l in lines.iter() {
        let len = l.len();
        let (r1, r2) = l.split_at(len/2);

        let mut common = ' ';

        for c in r1.chars() {
            if r2.contains(c) {
                common = c;
                break;
            }
        }
        let priority = get_priority(common);
        total += priority;
    }
    println!("Total: {}", total);
}


fn part_2() {
    let lines: Vec<String> = read_lines("input.txt");
    let mut total :u32 = 0;

    for chunk in lines.chunks_exact(3) {
        let mut common = ' ';

        for c in chunk[0].chars() {
            if chunk[1].contains(c) && chunk[2].contains(c) {
                common = c;
                break;
            }
        }

        let priority = get_priority(common);
        total += priority;
    }
    println!("Total: {}", total);
}


fn main() {
    part_1();
    part_2();
}
