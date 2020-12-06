use std::fs::File;
use std::io::{prelude::*, BufReader};


fn main() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let mut ids = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.as_str();
        let (row, col) = line.split_at(7);
        let row = row.replace("F", "0").replace("B", "1");
        let col = col.replace("L", "0").replace("R", "1");
        let row = usize::from_str_radix(&row, 2).unwrap();
        let col = usize::from_str_radix(&col, 2).unwrap();
        ids.push(row * 8 + col);
    }

    ids.sort();

    let mut prev = 36;

    for id in ids.iter() {
        if prev != id - 1 {
            println!("{:?}", id - 1);
        }
        prev = *id;
    }

    // println!("sum: {:?}", ids.first());

    Ok(())
}
