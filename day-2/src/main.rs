use std::fs::File;
use std::io::{prelude::*, BufReader};
extern crate regex;
use std::str::FromStr;

use regex::Regex;

fn main() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"(\d+)-(\d+) ([a-z]{1}): ([a-z]+)").unwrap();

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        match re.captures(&line) {
            Some(cap) => {
                let x = &cap[1];
                let x = usize::from_str(x).unwrap();
        
                let y = &cap[2];
                let y = usize::from_str(y).unwrap();
        
                let l = &cap[3];
                let l = l.chars().next().unwrap();
                let password = &cap[4];
                let password: Vec<_> = password.chars().collect();
        
                let char1 = password[x - 1];
                let char1 = char1 == l;

                let char2 = password[y - 1];
                let char2 = char2 == l;

                if char1 != char2 {
                    sum += 1;
                }
            }
            None => {
                println!("{}", line);
            }
        }

    }

    println!("sum: {:?}", sum);

    Ok(())
}


fn _part_1() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"(\d+)-(\d+) ([a-z]{1}): ([a-z]+)").unwrap();

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        match re.captures(&line) {
            Some(cap) => {
                let x = &cap[1];
                let x = usize::from_str(x).unwrap();
        
                let y = &cap[2];
                let y = usize::from_str(y).unwrap();
        
                let l = &cap[3];
                let password = &cap[4];
                let count = password.matches(l).count();
        
                if (x <= count) && count <= y {
                    sum += 1;
                }
            }
            None => {
                println!("{}", line);
            }
        }

    }

    println!("sum: {:?}", sum);

    Ok(())
}