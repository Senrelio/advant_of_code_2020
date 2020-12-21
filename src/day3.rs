use std::io::prelude::*;

#[allow(dead_code)]
fn solution(arrow: (u32, u32)) -> u32 {
    let input = "./input_day3";
    let file = std::fs::File::open(&input).unwrap();
    let file = std::io::BufReader::new(file);
    let lines = file
        .lines()
        .map(|l| {
            let line = l.unwrap();
            line.chars().map(|c| c == '#').collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();
    let line_length = lines[0].len();
    let mut x = 0usize;
    let mut y = 0usize;
    let mut count = 0;
    while y + (arrow.1 as usize) < lines.len() {
        x += arrow.0 as usize;
        y += arrow.1 as usize;
        let x_count = x % line_length;
        if lines[y][x_count] {
            count += 1;
        }
    }
    count
}

#[test]
fn run1() {
    let answer = solution((3, 1));
    println!("{}", answer);
}

#[test]
fn run2() {
    let inputs = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product: u32 = inputs.iter().map(|&input| solution(input)).product();
    println!("{}", product);
}
