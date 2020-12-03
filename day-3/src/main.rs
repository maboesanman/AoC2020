use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut pos = 0;

    for line in reader.lines().step_by(2) {
        let line: Vec<_> = line.unwrap().chars().collect();
        if line[pos] == '#' {
            sum += 1;
        }
        pos = (pos + 1) % line.len()
    }

    println!("sum: {:?}", sum);

    Ok(())
}
