use core::panic;
use std::{collections::VecDeque, fs::File};
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let mut numbers = VecDeque::<usize>::new();
    // let magic_num = 127;
    let magic_num = 2089807806;
    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let n = usize::from_str(&line).unwrap();
        numbers.push_back(n);
        sum += n;
        while sum > magic_num {
            if let Some(k) = numbers.pop_front() {
                sum -= k;
            }
        }
        if sum == magic_num {
            println!("{:?}", numbers);
            break;
        }
    }

    let final_sum = numbers.iter().min().unwrap() + numbers.iter().max().unwrap();

    println!("sum: {:?}", final_sum);

    Ok(())
}

fn _part_1() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let mut numbers = Vec::<usize>::new();
    let buffer_size = 25;
    let mut buffer: [usize; 25] = [0; 25];

    for (k, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let n = usize::from_str(&line).unwrap();
        // numbers.push(usize::from_str(&line).unwrap());
        if k >= buffer_size {
            let mut found = false;
            for i in 0 .. buffer_size {
                for j in i..buffer_size {
                    if buffer[i] + buffer[j] == n {
                        found = true;
                    }
                }
            }
            if !found {
                println!("{}", n);
            }
        }
        buffer[k % buffer_size] = n;
    }



    // println!("sum: {:?}", acc);

    Ok(())
}