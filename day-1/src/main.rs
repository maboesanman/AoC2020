use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let lines: Vec<usize> = reader.lines().map(|l| l.unwrap().parse::<usize>().unwrap()).collect();
    for x in lines.iter() {
        for y in lines.iter() {
            for z in lines.iter() {
                if x + y + z == 2020 {
                    println!("{}", x * y * z);
                    return Ok(());
                }
            }
        }
    }
    // for line in reader.lines() {
    //     println!("{}", line?);
    // }

    Ok(())
}
