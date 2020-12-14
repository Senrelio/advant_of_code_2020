use std::cmp::Ordering::{Equal, Greater, Less};

#[allow(dead_code)]
fn find_sum_2020(slice: &[u32]) -> Option<u32> {
    let mut slice = slice.to_owned();
    slice.sort_unstable();
    let mut l = 0usize;
    let mut r = slice.len() - 1;
    while l < r {
        match (slice[l] + slice[r]).cmp(&2020) {
            Less => {
                l += 1;
            }
            Equal => {
                return Some(slice[l] * slice[r]);
            }
            Greater => {
                r -= 1;
            }
        }
    }
    None
}

#[test]
fn run1() {
    use std::io::prelude::*;
    use std::io::BufReader;
    let input_path = "./input";
    let file = std::fs::File::open(&input_path).unwrap();
    let file = BufReader::new(file);
    let lines = file.lines();
    let lines = lines
        .filter_map(|l| l.unwrap().parse::<u32>().ok())
        .collect::<Vec<u32>>();
    println!("{}", find_sum_2020(&lines).unwrap());
}

#[allow(dead_code)]
fn three_sum(slice: &[u32]) -> Option<u32> {
    let mut slice = slice.to_owned();
    slice.sort_unstable();
    let mut pivot = 0usize;
    while pivot < slice.len() - 2 {
        let mut l = pivot + 1;
        let mut r = slice.len() - 1;
        while l < r {
            let sum = slice[pivot] + slice[l] + slice[r];
            match sum.cmp(&2020) {
                Less => l += 1,
                Equal => {
                    return Some(slice[pivot] * slice[l] * slice[r]);
                }
                Greater => r -= 1,
            }
        }
        pivot += 1;
    }
    None
}

#[test]
fn run2() {
    use std::io::prelude::*;
    use std::io::BufReader;
    let input_path = "./input";
    let file = std::fs::File::open(&input_path).unwrap();
    let file = BufReader::new(file);
    let lines = file.lines();
    let lines = lines
        .filter_map(|l| l.unwrap().parse::<u32>().ok())
        .collect::<Vec<u32>>();
    println!("{}", three_sum(&lines).unwrap());
}
