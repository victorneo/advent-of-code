use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_lines(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    return f.lines().map(|x| x.unwrap()).collect();
}

fn part_one() {
    let mut lines = read_lines("input.txt");
    let input = lines.pop().unwrap();
    let chars: Vec<char> = input.chars().collect();
    let end = chars.len() - 1;

    for index in 3..end {
        let current = chars[index];
        if (chars[index - 3] != current
            && chars[index - 2] != current
            && chars[index - 1] != current)
            && (chars[index - 1] != chars[index - 2] && chars[index - 1] != chars[index - 3])
            && (chars[index - 2] != chars[index - 3])
        {
            println!("{}", index + 1);
            break;
        }
    }
}

fn part_two() {
    let mut lines = read_lines("input.txt");
    let input = lines.pop().unwrap();
    let chars: Vec<char> = input.chars().collect();
    let end = chars.len() - 1;

    for index in 13..end {
        let mut char_set = HashSet::new();
        for i in (index - 13)..(index + 1) {
            char_set.insert(chars[i]);
        }
        if char_set.len() == 14 {
            println!("{}", index + 1);
            break;
        }
    }
}

fn main() {
    part_one();
    part_two();
}
