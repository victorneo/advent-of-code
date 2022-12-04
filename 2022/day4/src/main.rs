use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_lines(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    return f.lines().map(|x| x.unwrap()).collect();
}

fn part_1() {
    let lines = read_lines("input.txt");
    let mut total = 0;

    for l in lines.iter() {
        let mut groups = l.split(',');
        let pair1: Vec<u32> = groups
            .next()
            .unwrap()
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let pair2: Vec<u32> = groups
            .next()
            .unwrap()
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        if (pair1[0] >= pair2[0] && pair1[1] <= pair2[1])
            || (pair2[0] >= pair1[0] && pair2[1] <= pair1[1])
        {
            total += 1;
        }
    }

    println!("Total: {}", total);
}

fn part_2() {
    let lines = read_lines("input.txt");
    let mut total = 0;

    for l in lines.iter() {
        let mut groups = l.split(',');
        let pair1: Vec<u32> = groups
            .next()
            .unwrap()
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let pair2: Vec<u32> = groups
            .next()
            .unwrap()
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        if (pair1[0] >= pair2[0] && pair1[0] <= pair2[1])
            || (pair1[1] <= pair2[1] && pair1[1] >= pair2[0])
            || (pair2[0] >= pair1[0] && pair2[0] <= pair1[1])
            || (pair2[1] <= pair1[1] && pair2[1] >= pair1[0])
        {
            total += 1;
        }
    }

    println!("Total: {}", total);
}

fn main() {
    part_1();
    part_2();
}
