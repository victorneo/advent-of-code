use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_lines(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    return f.lines().map(|x| x.unwrap()).collect();
}

fn get_stacks(num_stacks: usize, stack_input: &Vec<String>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];

    for l in &stack_input[1..] {
        let mut index = 0;
        for i in 0..num_stacks {
            if i == 0 {
                index = 1;
            } else {
                index += 4;
            }

            let c = char::from_u32(l.as_bytes()[index] as u32).unwrap();
            if c != ' ' {
                stacks[i as usize].push(c);
            }
        }
    }

    return stacks;
}

fn part_one() {
    let mut stack_input = read_lines("input.txt");
    let mut split = 0;

    for l in stack_input.iter() {
        if l.trim() == "" {
            break;
        }
        split += 1;
    }

    let instructions = stack_input.split_off(split);
    stack_input.reverse();

    let num_stacks = stack_input
        .get(0)
        .unwrap()
        .split(' ')
        .filter(|x| !x.is_empty())
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut stacks = get_stacks(num_stacks, &stack_input);

    for l in &instructions[1..] {
        let mut i_split = l.split(' ');
        let num_pops = i_split.nth(1).unwrap().parse::<usize>().unwrap();
        let origin_stack = i_split.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let dest_stack = i_split.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        for _i in 0..num_pops {
            let c = stacks[origin_stack].pop().unwrap();
            stacks[dest_stack].push(c);
        }
    }

    let mut output = String::from("");
    for mut s in stacks {
        let c = s.pop();
        if c != None {
            output.push(c.unwrap());
        }
    }
    println!("{}", output);
}

fn part_two() {
    let mut stack_input = read_lines("input.txt");
    let mut split = 0;

    for l in stack_input.iter() {
        if l.trim() == "" {
            break;
        }
        split += 1;
    }

    let instructions = stack_input.split_off(split);
    stack_input.reverse();

    let num_stacks = stack_input
        .get(0)
        .unwrap()
        .split(' ')
        .filter(|x| !x.is_empty())
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut stacks = get_stacks(num_stacks, &stack_input);

    for l in &instructions[1..] {
        let mut i_split = l.split(' ');
        let num_pops = i_split.nth(1).unwrap().parse::<usize>().unwrap();
        let origin_stack = i_split.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let dest_stack = i_split.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        let mut temp_stack: Vec<char> = vec![];
        for _i in 0..num_pops {
            let c = stacks[origin_stack].pop().unwrap();
            temp_stack.push(c);
        }

        while !temp_stack.is_empty() {
            stacks[dest_stack].push(temp_stack.pop().unwrap());
        }
    }

    let mut output = String::from("");
    for mut s in stacks {
        let c = s.pop();
        if c != None {
            output.push(c.unwrap());
        }
    }
    println!("{}", output);
}

fn main() {
    part_one();
    part_two();
}
