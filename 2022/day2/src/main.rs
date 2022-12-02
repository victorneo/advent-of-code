use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;


fn read_lines(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    return f.lines().map(|x| x.unwrap()).collect();
}


fn part_1() {
    let lines : Vec<String> = read_lines("input.txt");
    let mut total_score = 0;
    let mut curr_score = 0;

    for line in lines.iter() {
        let mut split = line.split(' ');
        let opponent = split.next().unwrap();
        let mine = split.next().unwrap();

        curr_score += match mine {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };

        curr_score += match (opponent, mine) {
            ("A", "X")|("B", "Y")|("C","Z") => 3,
            ("A", "Y")|("B", "Z")|("C","X") => 6,
            _ => 0,
        };

        total_score += curr_score;
        curr_score = 0;
    }

    println!("Total: {}", total_score);
}

fn part_2() {
    let lines : Vec<String> = read_lines("input.txt");
    let mut total_score = 0;
    let mut curr_score = 0;

    for line in lines.iter() {
        let mut split = line.split(' ');
        let opponent = split.next().unwrap();
        let choice = split.next().unwrap();

        let mine = match (opponent, choice) {
            ("A","Y")|("C", "Z")|("B", "X") => "X",
            ("B","Y")|("A", "Z")|("C", "X") => "Y",
            ("C","Y")|("B", "Z")|("A", "X") => "Z",
            _ => "X",
        };

        curr_score += match (opponent, mine) {
            ("A", "X")|("B", "Y")|("C","Z") => 3,
            ("A", "Y")|("B", "Z")|("C","X") => 6,
            _ => 0,
        };

        curr_score += match mine {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };

        total_score += curr_score;
        curr_score = 0;
    }

    println!("Total: {}", total_score);
}


fn main() {
    part_1();
    part_2();
}
