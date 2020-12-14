use std::io::prelude::*;
#[derive(Debug)]
struct Line {
    first: usize,
    second: usize,
    c: char,
    pwd: String,
}

#[allow(dead_code)]
impl Line {
    pub fn pass_1(&self) -> bool {
        let count = self.pwd.chars().filter(|c| c == &self.c).count();
        count >= self.first && count <= self.second
    }
    pub fn pass_2(&self) -> bool {
        let bytes = self.pwd.as_bytes();
        (bytes[self.first - 1] as char == self.c) ^ (bytes[self.second - 1] as char == self.c)
    }
}

#[allow(dead_code)]
fn parse_line() -> Vec<Line> {
    let input = "./input_day2";
    let file = std::fs::File::open(input).unwrap();
    let file = std::io::BufReader::new(file);
    let file = file.lines();
    file.into_iter()
        .map(|l| {
            let line = l.as_ref().unwrap();
            let i_minus = line.chars().position(|c| c == '-').unwrap();
            let i_space = line.chars().position(|c| c == ' ').unwrap();
            let i_colon = line.chars().position(|c| c == ':').unwrap();
            let first = line[..i_minus].parse::<usize>().unwrap();
            let second = line[i_minus + 1..i_space].parse::<usize>().unwrap();
            let c = line.as_bytes()[i_colon - 1] as char;
            let pwd = line[i_colon + 2..].to_string();
            Line {
                first,
                second,
                c,
                pwd,
            }
        })
        .collect::<Vec<Line>>()
}

#[test]
fn run1() {
    let lines = parse_line();
    let count = lines.into_iter().filter(|l| l.pass_1()).count();
    println!("{}", count);
}

#[test]
fn run2() {
    let lines = parse_line();
    let count = lines.into_iter().filter(|l| l.pass_2()).count();
    println!("{}", count);
}
