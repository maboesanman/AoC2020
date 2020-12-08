use core::panic;
use std::fs::File;
use std::io::{prelude::*, BufReader};
extern crate regex;
use std::str::FromStr;

use regex::Regex;

fn _part_1() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"(nop|acc|jmp) (\+|-)(\d+)").unwrap();

    let mut instructions = Vec::<(String, isize, bool)>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        match re.captures(&line) {
            Some(cap) => {
                let op = &cap[1];
        
                let sgn = &cap[2];
        
                let n = &cap[3];
                let mut n = isize::from_str(n).unwrap();
                if sgn == "-" {
                    n *= -1;
                }

                instructions.push((op.to_string(), n, false));
            }
            None => {
                println!("{}", line);
            }
        }
    }

    let mut pos: usize = 0;
    let mut acc: isize = 0;
    loop {
        match instructions.get_mut(pos) {
            Some((op, n, visited)) => {
                if *visited {
                    break;
                }
                match op.as_str() {
                    "acc" => {
                        acc += *n;
                        pos += 1;
                    }
                    "jmp" => {
                        pos = ((pos as isize) + *n) as usize;
                    }
                    "nop" => {
                        pos += 1;
                    },
                    _ => panic!(),
                };
                *visited = true;
            }
            None => panic!()
        }
    }

    println!("sum: {:?}", acc);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"(nop|acc|jmp) (\+|-)(\d+)").unwrap();

    let mut instructions = Vec::<(String, isize, bool)>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        match re.captures(&line) {
            Some(cap) => {
                let op = &cap[1];
        
                let sgn = &cap[2];
        
                let n = &cap[3];
                let mut n = isize::from_str(n).unwrap();
                if sgn == "-" {
                    n *= -1;
                }

                instructions.push((op.to_string(), n, false));
            }
            None => {
                println!("{}", line);
            }
        }
    }

    let mut flip_pos = 0;
    let mut pos: usize;
    let mut acc: isize;
    'outer: loop {
        // if instructions.get(flip_pos).unwrap().0 == "acc".to_string() {
        //     continue;
        // }

        pos = 0;
        acc = 0;

        for inst in instructions.iter_mut() {
            inst.2 = false;
        }

        let x = &mut instructions.get_mut(flip_pos).unwrap().0;
        if x == "nop" {
            *x = "jmp".to_string();
        } else if x == "jmp" {
            *x = "nop".to_string();
        }

        'inner: loop {
            match instructions.get_mut(pos) {
                Some((op, n, visited)) => {
                    if *visited {
                        break 'inner;
                    }
                    match op.as_str() {
                        "acc" => {
                            acc += *n;
                            pos += 1;
                        }
                        "jmp" => {
                            pos = ((pos as isize) + *n) as usize;
                        }
                        "nop" => {
                            pos += 1;
                        },
                        _ => panic!(),
                    };
                    *visited = true;
                }
                None => {
                    if pos == instructions.len() {
                        break 'outer
                    } else {
                        break 'inner
                    }
                }
            }
        }

        let x = &mut instructions.get_mut(flip_pos).unwrap().0;
        if x == "nop" {
            *x = "jmp".to_string();
        } else if x == "jmp" {
            *x = "nop".to_string();
        }
        flip_pos += 1;
    }

    println!("sum: {:?}", acc);

    Ok(())
}