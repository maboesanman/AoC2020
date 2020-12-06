use std::{collections::HashSet, fs::File, collections::HashMap};
use std::io::{prelude::*, BufReader};

fn _part_1() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let mut groups: Vec<HashSet<char>> = Vec::new();
    let mut current_group: HashSet<char> = HashSet::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            groups.push(current_group);
            current_group = HashSet::new();
            continue;
        }
        for c in line.chars() {
            current_group.insert(c);
        }
    }

    groups.push(current_group);

    let sum: usize = groups.iter().map(|g| {
        g.len()
    }).sum();

    println!("sum: {:?}", sum);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let mut groups: Vec<(usize, HashMap<char, usize>)> = Vec::new();
    let mut current_group_size: usize = 0;
    let mut current_group: HashMap<char, usize> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            groups.push((current_group_size, current_group));
            current_group_size = 0;
            current_group = HashMap::new();
            continue;
        }
        for c in line.chars() {
            match current_group.get_mut(&c) {
                Some(count) => {
                    *count += 1;
                }
                None => {
                    current_group.insert(c, 1);
                }
            }
        }
        current_group_size += 1;
    }

    groups.push((current_group_size, current_group));

    let sum: usize = groups.iter().map(|(n, g)| {
        g.iter().filter(|(_, count)| *n == **count).count()
    }).sum();

    println!("sum: {:?}", sum);

    Ok(())
}