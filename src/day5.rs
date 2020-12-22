use std::io::prelude::*;
pub fn verify(seq: Vec<char>) -> u32 {
    let row_seq = &seq[..7];
    let col_seq = &seq[7..];
    let row_num = row_seq
        .iter()
        .map(|c| match c {
            'F' => 0,
            'B' => 1,
            e => panic!("unexpected char: {}", e),
        })
        .fold(None, |p, c| {
            if let Some(acc) = p {
                Some(acc << 1 | c)
            } else {
                Some(c)
            }
        })
        .unwrap();
    let col_num = col_seq
        .iter()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            e => panic!("unexpected char: {}", e),
        })
        .fold(None, |p, c| {
            if let Some(acc) = p {
                Some(acc << 1 | c)
            } else {
                Some(c)
            }
        })
        .unwrap();
    row_num * 8 + col_num
}

#[test]
fn run1() {
    let input = std::fs::File::open("./input_day5").unwrap();
    let max = std::io::BufReader::new(input)
        .lines()
        .into_iter()
        .map(|l| verify(l.unwrap().chars().collect()))
        .max()
        .unwrap();
    println!("{}", max);
}

#[test]
fn run2() {
    let input = std::fs::File::open("./input_day5").unwrap();
    let mut present: Vec<u32> = std::io::BufReader::new(input)
        .lines()
        .into_iter()
        .map(|l| verify(l.unwrap().chars().collect()))
        .collect();
    present.sort_unstable();
    let bingo = present
        .windows(2)
        .find(|w| w[0] + 1 != w[1])
        .unwrap()
        .get(0)
        .unwrap()
        + 1;
    println!("{}", bingo);
}
