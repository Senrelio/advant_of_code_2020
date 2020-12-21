use regex::Regex;
use std::{collections::HashMap, io::prelude::*};

#[allow(dead_code)]
fn solution(input: String) -> usize {
    input
        .split("\n\n")
        .into_iter()
        .filter(|r| Passport::verify(r).is_ok())
        .count()
}

#[allow(dead_code)]
struct Passport<'a> {
    byr: &'a str,
    iyr: &'a str,
    eyr: &'a str,
    hgt: &'a str,
    hcl: &'a str,
    ecl: &'a str,
    pid: &'a str,
    cid: Option<&'a str>,
}

use once_cell::sync::Lazy;
static BYR: Lazy<Regex> = Lazy::new(|| Regex::new(r"(19[2-9]\d|200[0-2])\b").unwrap());
static IYR: Lazy<Regex> = Lazy::new(|| Regex::new(r"20(1\d|20)\b").unwrap());
static EYR: Lazy<Regex> = Lazy::new(|| Regex::new(r"20(2\d|30)\b").unwrap());
static HGT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(1([5-8]\d|9[0-3])cm|(59|6\d|7[0-6])in)\b").unwrap());
static HCL: Lazy<Regex> = Lazy::new(|| Regex::new(r"#[0-9a-f]{6}\b").unwrap());
static ECL: Lazy<Regex> = Lazy::new(|| Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap());
static PID: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d{9}\b").unwrap());

#[allow(dead_code)]
impl<'a> Passport<'a> {
    pub fn verify(record: &'a str) -> Result<Passport<'a>, ()> {
        let map = record
            .split_ascii_whitespace()
            .map(|e| {
                let mut i = e.split(':');
                (i.next().unwrap().trim(), i.next().unwrap().trim())
            })
            .collect::<HashMap<&str, &str>>();
        Passport::from_map(map)
    }
    pub fn validate(&self) -> bool {
        BYR.is_match(&self.byr.trim())
            && IYR.is_match(&self.iyr.trim())
            && EYR.is_match(&self.eyr.trim())
            && HGT.is_match(&self.hgt.trim())
            && HCL.is_match(&self.hcl.trim())
            && ECL.is_match(&self.ecl.trim())
            && PID.is_match(&self.pid.trim())
    }
    pub fn from_map(map: HashMap<&'a str, &'a str>) -> Result<Passport<'a>, ()> {
        let byr = map.get("byr").ok_or(())?;
        let iyr = map.get("iyr").ok_or(())?;
        let eyr = map.get("eyr").ok_or(())?;
        let hgt = map.get("hgt").ok_or(())?;
        let hcl = map.get("hcl").ok_or(())?;
        let ecl = map.get("ecl").ok_or(())?;
        let pid = map.get("pid").ok_or(())?;
        let cid = map.get("cid");
        let passport = Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid: {
                if cid.is_some() {
                    Some(cid.unwrap())
                } else {
                    None
                }
            },
        };
        if passport.validate() {
            Ok(passport)
        } else {
            Err(())
        }
    }
}

#[test]
fn run1() {
    let file = std::fs::File::open("./input_day4").unwrap();
    let mut file = std::io::BufReader::new(file);
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let count = solution(buf);
    println!("{}", count);
}
#[test]
fn invalid() {
    let inputs = [
        "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
",
        "
iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946
",
        "
hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

",
        "
hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007",
    ];
    assert!(inputs
        .iter()
        .find(|input| Passport::verify(input).is_ok())
        .is_none())
}
#[test]
fn valid() {
    let inputs = [
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
    hcl:#623a2f
    ",
        "
    eyr:2029 ecl:blu cid:129 byr:1989
    iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
    ",
        "
    hcl:#888785
    hgt:164cm byr:2001 iyr:2015 cid:88
    pid:545766238 ecl:hzl
    eyr:2022
    ",
        "    iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
    ];
    assert!(inputs
        .iter()
        .find(|input| Passport::verify(input).is_err())
        .is_none())
}
