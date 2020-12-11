use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let mut numbers = Vec::<usize>::new();
    numbers.push(0);

    for line in reader.lines() {
        let line = line.unwrap();
        let n = usize::from_str(&line).unwrap();
        numbers.push(n);
    }

    numbers.sort();

    numbers.push(numbers.last().unwrap() + 3);

    let mut numbers: Vec<_> = numbers.into_iter().map(|x| (x, 0u128)).collect();

    let mut i = 1;
    let (_, n) = numbers.first_mut().unwrap();
    *n = 1;


    while i < numbers.len() {
        let (current, _) = numbers.get(i).unwrap();
        let mut n = 0u128;
        for j in 1..4 {
            if j > i {
                continue;
            }
            if let Some((previous, k)) = numbers.get(i - j) {
                if current - previous <= 3 {
                    n += k;
                }
            }
        }
        let (_, n2) = numbers.get_mut(i).unwrap();
        *n2 = n;

        println!("{}", n);

        i += 1;
    }

    println!("{:?}", numbers.last().unwrap().1);

    Ok(())
}


fn _part_1() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let mut numbers = Vec::<usize>::new();
    numbers.push(0);

    for line in reader.lines() {
        let line = line.unwrap();
        let n = usize::from_str(&line).unwrap();
        numbers.push(n);
    }

    numbers.sort();

    numbers.push(numbers.last().unwrap() + 3);

    println!("{:?}", numbers);

    let mut differences = Vec::<usize>::new();

    let iter_1 = numbers.iter();
    let iter_2 = numbers.iter().skip(1);

    let mut num_1 = 0;
    let mut num_3 = 0;

    for (x, y) in iter_1.zip(iter_2) {
        let d = y - x;
        if d == 1 {
            num_1 += 1;
        } else 
        if d == 3 {
            num_3 += 1;
        } 
    }



    println!("{:?} * {:?} = {:?}, ", num_1,  num_3, num_1 * num_3);

    Ok(())
}
