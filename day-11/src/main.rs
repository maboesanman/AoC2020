use core::panic;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

fn main() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let mut seats: Vec<Vec<Seat>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.chars();
        let line = line.map(|c| match c {
            '.' => Seat::Floor,
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            _ => panic!()
        });
        seats.push(line.collect());
    }

    let (mut x, mut y) = (0usize, 0usize);

    let width = seats.first().unwrap().len();
    let height = seats.len();


    let mut changed = true;
    // print_seats(&seats);
    'outer: while changed == true {
        changed = false;
        x = 0;
        y = 0;
        let old_seats = seats.clone();
        'inner: loop {
            let row = match old_seats.get(y) {
                Some(row) => row,
                None => {
                    panic!();
                }
            };
            let seat = match row.get(x) {
                Some(item) => item,
                None => {
                    panic!();
                }
            };
    
            let next = next_2(&old_seats, x, y);
    
            if next != *seat {
                changed = true;
                *seats.get_mut(y).unwrap().get_mut(x).unwrap() = next;
            }
    
            x += 1;
            if x == width {
                x = 0;
                y += 1;
                if y == height {
                    break 'inner;
                }
            }
        }
        // print_seats(&seats);

    }

    let sum = seats.iter().map(|row| {
        row.iter().map(|item| match item {
            Seat::Empty => 0usize,
            Seat::Occupied => 1usize,
            Seat::Floor => 0usize,
        }).sum::<usize>()
    }).sum::<usize>();

    println!("{:?}", sum);

    Ok(())
}

fn _part_1() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let mut seats: Vec<Vec<Seat>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.chars();
        let line = line.map(|c| match c {
            '.' => Seat::Floor,
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            _ => panic!()
        });
        seats.push(line.collect());
    }

    let (mut x, mut y) = (0usize, 0usize);

    let width = seats.first().unwrap().len();
    let height = seats.len();


    let mut changed = true;
    print_seats(&seats);
    'outer: while changed == true {
        changed = false;
        x = 0;
        y = 0;
        let old_seats = seats.clone();
        'inner: loop {
            let row = match old_seats.get(y) {
                Some(row) => row,
                None => {
                    panic!();
                }
            };
            let seat = match row.get(x) {
                Some(item) => item,
                None => {
                    panic!();
                }
            };
    
            let next = next(&old_seats, x, y);
    
            if next != *seat {
                changed = true;
                *seats.get_mut(y).unwrap().get_mut(x).unwrap() = next;
            }
    
            x += 1;
            if x == width {
                x = 0;
                y += 1;
                if y == height {
                    break 'inner;
                }
            }
        }
        print_seats(&seats);

    }

    let sum = seats.iter().map(|row| {
        row.iter().map(|item| match item {
            Seat::Empty => 0usize,
            Seat::Occupied => 1usize,
            Seat::Floor => 0usize,
        }).sum::<usize>()
    }).sum::<usize>();

    println!("{:?}", sum);

    Ok(())
}

fn next(seats: &Vec<Vec<Seat>>, x: usize, y: usize) -> Seat {
    let row = match seats.get(y) {
        Some(row) => row,
        None => {
            panic!();
        }
    };
    let seat = match row.get(x) {
        Some(item) => item,
        None => {
            panic!();
        }
    };

    match seat {
        Seat::Empty => {
            if surrounding_count(seats, x, y) == 0 {
                Seat::Occupied
            } else {
                Seat::Empty
            }
        },
        Seat::Occupied => {
            if surrounding_count(seats, x, y) >= 4 {
                Seat::Empty
            } else {
                Seat::Occupied
            }
        },
        Seat::Floor => Seat::Floor,
    }
}

fn next_2(seats: &Vec<Vec<Seat>>, x: usize, y: usize) -> Seat {
    let row = match seats.get(y) {
        Some(row) => row,
        None => {
            panic!();
        }
    };
    let seat = match row.get(x) {
        Some(item) => item,
        None => {
            panic!();
        }
    };

    match seat {
        Seat::Empty => {
            if visible_count(seats, x, y) == 0 {
                Seat::Occupied
            } else {
                Seat::Empty
            }
        },
        Seat::Occupied => {
            if visible_count(seats, x, y) >= 5 {
                Seat::Empty
            } else {
                Seat::Occupied
            }
        },
        Seat::Floor => Seat::Floor,
    }
}

fn surrounding_count(seats: &Vec<Vec<Seat>>, x: usize, y: usize) -> usize {
    let width = seats.first().unwrap().len();
    let height = seats.len();
    let min_x = if x == 0 {
        0
    } else {
        x - 1
    };

    let max_x = if x == width - 1 {
        x
    } else {
        x + 1
    };

    let min_y = if y == 0 {
        0
    } else {
        y - 1
    };

    let max_y = if y == height - 1 {
        y
    } else {
        y + 1
    };

    let mut count = 0;

    for sub_y in min_y..(max_y + 1) {
        for sub_x in min_x..(max_x + 1) {
            if sub_x == x && sub_y == y {
                continue;
            }
            let row = match seats.get(sub_y) {
                Some(row) => row,
                None => {
                    panic!();
                }
            };
            let item = match row.get(sub_x) {
                Some(item) => item,
                None => {
                    panic!();
                }
            };

            if let Seat::Occupied = item {
                count += 1;
            }
        }
    }

    count
}

fn visible_count(seats: &Vec<Vec<Seat>>, x: usize, y: usize) -> usize {
    let x = x as i64;
    let y = y as i64;
    let mut count = 0;

    for dy in -1..2 {
        for dx in -1..2 {
            let mut dist = 1;
            'inner: loop {
                if dx == 0 && dy == 0 {
                    break 'inner;
                }
                let q_x = x + dist * dx;
                let q_y = y + dist * dy;
                if q_x < 0 || q_y < 0 {
                    break 'inner;
                }
                let row = match seats.get(q_y as usize) {
                    Some(row) => row,
                    None => {
                        break 'inner;
                    }
                };
                let item = match row.get(q_x as usize) {
                    Some(item) => item,
                    None => {
                        break 'inner;
                    }
                };
    
                match item {
                    Seat::Empty => {
                        break 'inner;
                    }
                    Seat::Occupied => {
                        count += 1;
                        break 'inner
                    }
                    Seat::Floor => {}
                }
                dist += 1;
            }
        }
    }

    count
}

fn print_seats(seats: &Vec<Vec<Seat>>) {
    for row in seats {
        for item in row {
            print!("{}", match item {
                Seat::Empty => "L",
                Seat::Occupied => "#",
                Seat::Floor => ".",
            })
        }
        println!();
    }
    println!();
}