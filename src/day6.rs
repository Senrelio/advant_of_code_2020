use std::collections::HashSet;
pub fn solution(input: Vec<Vec<char>>) -> usize {
    input.iter().map(|v| handle_group_input(v)).sum()
}
pub fn solution2(input: Vec<Vec<&str>>) -> usize {
    input.iter().map(|v| handle_group_input2(v)).sum()
}

pub fn handle_group_input(input: &[char]) -> usize {
    input.iter().collect::<HashSet<_>>().len()
}
pub fn handle_group_input2(input: &[&str]) -> usize {
    input
        .iter()
        .fold(None, |p: Option<HashSet<char>>, c| {
            if let Some(v) = p {
                Some(
                    c.chars()
                        .filter(|e| v.contains(e))
                        .map(|e| e.to_owned())
                        .collect(),
                )
            } else {
                Some(c.chars().collect())
            }
        })
        .unwrap()
        .len()
}

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::{fs::File, io::BufReader};

    use super::*;
    const INPUT_PATH: &str = "./input_day6";
    #[test]
    fn run1() {
        let mut input = BufReader::new(File::open(INPUT_PATH).unwrap());
        let mut buf = String::new();
        input.read_to_string(&mut buf).unwrap();
        let input: Vec<Vec<char>> = buf
            .trim()
            .split("\n\n")
            .map(|s| s.split('\n').map(|str| str.chars()).flatten().collect())
            .collect();
        let answer = solution(input);
        dbg!(answer);
    }
    #[test]
    fn run2() {
        let mut input = BufReader::new(File::open(INPUT_PATH).unwrap());
        let mut buf = String::new();
        input.read_to_string(&mut buf).unwrap();
        let input: Vec<Vec<&str>> = buf
            .trim()
            .split("\n\n")
            .map(|s| s.split('\n').collect())
            .collect();
        let answer = solution2(input);
        dbg!(answer);
    }
}
