use std::{collections::HashMap, fs::File, collections::HashSet};
use std::io::{prelude::*, BufReader};
extern crate regex;
use std::str::FromStr;

use regex::Regex;

fn _part_1() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    let re1 = Regex::new(r"([a-z ]+) bags contain .*").unwrap();
    let re2 = Regex::new(r"(\d+) ([a-z ]+) bag(s?)").unwrap();

    let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let bag_name = &re1.captures(&line).unwrap()[1];
        // println!("{}", bag_name);
        for cap in re2.captures_iter(&line) {

            let bag_dep = (
                &cap[2],
                usize::from_str(&cap[1]).unwrap(),
            );

            println!("{:?}", bag_dep);

            match dependencies.get_mut(&bag_dep.0.to_string()) {
                Some(vec) => {
                    vec.push(bag_name.to_string());
                }
                None => {
                    dependencies.insert(bag_dep.0.to_string(), vec![bag_name.to_string()]);
                }
            }
        }
    }

    for x in dependencies.iter() {
        println!("{:?}", x);
    }

    let mut searched: HashSet<String> = HashSet::new();
    let mut to_search: HashSet<String> = HashSet::new();
    to_search.insert("shiny gold".to_string());

    while !to_search.is_empty() {
        let mut to_search_next: Vec<String> = Vec::new();
        for item in to_search.drain() {
            println!("processing {:?}", item);
            searched.insert(item.clone());
            if let Some(deps) = dependencies.get(&item) {
                for dep in deps {
                    if !searched.contains(dep) {
                        to_search_next.push(dep.to_string());
                    } else {
                        println!("duplicate");
                    }
                }
            }
        }
        to_search.extend(to_search_next);
    }

    println!("sum: {:?}", searched.len() - 1);

    Ok(())
}


fn main() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    let re1 = Regex::new(r"([a-z ]+) bags contain .*").unwrap();
    let re2 = Regex::new(r"(\d+) ([a-z ]+) bag(s?)").unwrap();

    let mut dependencies: HashMap<String, Vec<(usize, String)>> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let bag_name = &re1.captures(&line).unwrap()[1];
        // println!("{}", bag_name);
        for cap in re2.captures_iter(&line) {

            let bag_dep = (
                usize::from_str(&cap[1]).unwrap(),
                cap[2].to_string(),
            );

            // println!("{:?}", bag_dep);

            match dependencies.get_mut(&bag_name.to_string()) {
                Some(vec) => {
                    vec.push(bag_dep);
                }
                None => {
                    dependencies.insert(bag_name.to_string(), vec![bag_dep]);
                }
            }
        }
    }

    let mut calculated: HashMap<String, usize> = HashMap::new();
    

    println!("sum: {:?}", calculate("shiny gold", &mut calculated, &dependencies) - 1);

    Ok(())
}


fn calculate(bag_type: &str, calculated: &mut HashMap<String, usize>, dependencies: &HashMap<String, Vec<(usize, String)>>) -> usize {
    if let Some(value) = calculated.get(bag_type) {
        *value
    } else {
        if let Some(vec) = dependencies.get(bag_type) {
            let dep_sum: usize = vec.iter().map(|(count, bag)| count * calculate(bag, calculated, dependencies)).sum();
            1 + dep_sum
        } else {
            1
        }
    }
}