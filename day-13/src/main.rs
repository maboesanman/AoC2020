use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;


fn main() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let mut busses: Vec<(usize, usize)> = Vec::new();

    let mut lines = reader.lines();

    let _ = usize::from_str(&(lines.next().unwrap().unwrap()));
    for (i, t) in lines.next().unwrap().unwrap().split(",").enumerate() {
        // println!("{}", t);
        if t == "x" {
            continue;
        }
        let n_i = isize::from_str(t).unwrap();
        let a_i = (i as isize) * -1;
        let a_i = ((a_i % n_i) + n_i) % n_i;
        let n_i = n_i as usize;
        let a_i = a_i as usize;
        busses.push((n_i, a_i));
    }

    // let n: usize = busses.iter().map(|(n, _)| n).product();
    busses.sort_by(|(n_1, _), (n_2, _)| n_1.cmp(n_2).reverse());

    let mut n = 1;
    let mut a = 0;
    let mut k;

    'outer: for (n_i, a_i)in busses.iter() {
        k = 0;
        loop {
            let possible = a + n * k;
            if *a_i == possible % n_i {
                n *= n_i;
                a = possible;
                continue 'outer;
            }
            k += 1;
        }
    }

    println!("n: {}, a: {}", n, a);


    Ok(())
}

fn _part_1() -> std::io::Result<()> {
    let n =-1005595;
    let times = vec![41,37,557,29,13,17,23,419,19];

    for x in times {
        println!("n: {:?}, m: {:?}", x, ((n % x) + x) % x);
    }

    Ok(())
}