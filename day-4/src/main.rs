use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
extern crate regex;

use regex::Regex;

fn main() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let mut passports: Vec<HashMap<String, String>> = Vec::new();

    let mut current: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            passports.push(current);
            current = HashMap::new();
            continue;
        }
        for s in line.split(" ") {
            let kv: Vec<_> = s.split(":").collect();
            if kv.len() != 2 {
                println!("{}", line);
            }
            current.insert(kv[0].to_string(), kv[1].to_string());
        }
    }

    passports.push(current);



    let sum = passports.iter().filter(|passport| {
        if !(passport.contains_key("byr") && byr_valid(&passport["byr"])) {
            return false;
        }
        if !(passport.contains_key("iyr") && iyr_valid(&passport["iyr"])) {
            return false;
        }
        if !(passport.contains_key("eyr") && eyr_valid(&passport["eyr"])) {
            return false;
        }
        if !(passport.contains_key("hgt") && hgt_valid(&passport["hgt"])) {
            return false;
        }
        if !(passport.contains_key("hcl") && hcl_valid(&passport["hcl"])) {
            return false;
        }
        if !(passport.contains_key("ecl") && ecl_valid(&passport["ecl"])) {
            return false;
        }
        if !(passport.contains_key("pid") && pid_valid(&passport["pid"])) {
            return false;
        }
        
        return true;
    }).count();

    println!("sum: {:?}", sum);

    Ok(())
}

fn byr_valid(input: &str) -> bool {
    let x = match input.parse::<usize>() {
        Ok(int) => 1920 <= int && int <= 2002,
        Err(_) => false
    };
    if !x {
        println!("byr:{}", input);
    }
    x
}

fn iyr_valid(input: &str) -> bool {
    let x = match input.parse::<usize>() {
        Ok(int) => 2010 <= int && int <= 2020,
        Err(_) => false
    };
    if !x {
        println!("iyr:{}", input);
    }
    x
}

fn eyr_valid(input: &str) -> bool {
    let x = match input.parse::<usize>() {
        Ok(int) => 2020 <= int && int <= 2030,
        Err(_) => false
    };
    if !x {
        println!("eyr:{}", input);
    }
    x
}

fn hgt_valid(input: &str) -> bool {
    let re = Regex::new(r"(\d+)(cm|in)").unwrap();
    let x = match re.captures(&input) {
        Some(cap) => {
            let num = &cap[1];
            let num = num.parse::<usize>().unwrap_or(0);
            let dim = &cap[2];

            if dim == "cm" {
                150 <= num && num <= 193
            } else {
                59 <= num && num <= 76
            }
        }
        None => false
    };
    if !x {
        println!("hgt:{}", input);
    }
    x
}

fn hcl_valid(input: &str) -> bool {
    let re = Regex::new(r"#[0-9a-f]{6}").unwrap();
    let x = re.is_match(&input);
    if !x {
        println!("hcl:{}", input);
    }
    x
}

fn ecl_valid(input: &str) -> bool {
    let re = Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
    let x = re.is_match(&input);
    if !x {
        println!("ecl:{}", input);
    }
    x
}

fn pid_valid(input: &str) -> bool {
    let re = Regex::new(r"\d{9}").unwrap();
    let x = re.is_match(&input);
    if !x {
        println!("pid:{}", input);
    }
    x
}