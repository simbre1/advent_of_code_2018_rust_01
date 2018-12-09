use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("D:\\dev\\advent_of_code_2018\\rust-01\\input.txt")
        .expect("peut");

    let changes: Vec<i32> = contents
        .lines()
        .map(|line| line.parse::<i32>())
        .filter_map(Result::ok)
        .collect();

    println!("part one:         {}", part_one(&changes));
    println!("part one generic: {}", part_one_generic(&changes));

    print!("part two:         ");
    match part_two(&changes) {
        Some(i) => println!("{}", i),
        None => println!("none"),
    }
    print!("part two generic: ");
    match part_two_generic(&changes) {
        Some(i) => println!("{}", i),
        None => println!("none"),
    }
}

fn part_one(changes: &Vec<i32>) -> i32 {
    changes.iter().sum()
}

fn part_one_generic<T>(changes: &[T]) -> T
    where T: std::iter::Sum<T>
    + std::clone::Clone {
    changes.iter().cloned().sum()
}

fn part_two(changes: &Vec<i32>) -> Option<i32> {

    if changes.is_empty() {
        return None;
    }

    let mut freqs = HashSet::new();
    let mut freq = 0;

    loop {
        for change in changes.iter() {
            freq += change;
            if freqs.contains(&freq) {
                return Some(freq);
            }
            freqs.insert(freq);
        }
    }
}

fn part_two_generic<T>(changes: &[T]) -> Option<T>
    where T: std::ops::AddAssign
    + std::hash::Hash
    + std::cmp::Eq
    + std::default::Default
    + std::clone::Clone {

    if changes.is_empty() {
        return None;
    }

    let mut freqs = HashSet::new();
    let mut freq = T::default();

    loop {
        for change in changes.iter() {
            freq += change.clone();
            if freqs.contains(&freq) {
                return Some(freq);
            }
            freqs.insert(freq.clone());
        }
    }
}
