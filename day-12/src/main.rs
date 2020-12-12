use core::panic;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Instruction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

fn main() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let mut instructions: Vec<(Instruction, isize)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.chars();
        let instr = line.next().unwrap();
        let num: String = line.collect();
        let num = isize::from_str(&num).unwrap();

        let instr = match instr {
            'N' => Instruction::North,
            'S' => Instruction::South,
            'E' => Instruction::East,
            'W' => Instruction::West,
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            'F' => Instruction::Forward,
            _ => panic!()
        };
        instructions.push((instr, num));
    }

    let mut x = 0isize;
    let mut y = 0isize;
    let mut w_x = 10isize;
    let mut w_y = 1isize;

    for (instr, num) in instructions {
        match instr {
            Instruction::North => {
                w_y += num;
            }
            Instruction::South => {
                w_y -= num;
            }
            Instruction::East => {
                w_x += num;
            }
            Instruction::West => {
                w_x -= num;
            }
            Instruction::Left => {
                let times = match num {
                    90 => 1,
                    180 => 2,
                    270 => 3,
                    _ => panic!(),
                };

                for _ in 0..times {
                    std::mem::swap(&mut w_x, &mut w_y);
                    w_x = -w_x;
                }
            }
            Instruction::Right => {
                let times = match num {
                    90 => 3,
                    180 => 2,
                    270 => 1,
                    _ => panic!(),
                };

                for _ in 0..times {
                    std::mem::swap(&mut w_x, &mut w_y);
                    w_x = -w_x;
                }
            }
            Instruction::Forward => {
                for _ in 0..num {
                    x += w_x;
                    y += w_y;
                }
            }
        }
    }

    println!("{:?}", x.abs() + y.abs());

    Ok(())
}

fn _part_1() -> std::io::Result<()> {
    let file = File::open("in")?;
    let reader = BufReader::new(file);

    let mut instructions: Vec<(Instruction, isize)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.chars();
        let instr = line.next().unwrap();
        let num: String = line.collect();
        let num = isize::from_str(&num).unwrap();

        let instr = match instr {
            'N' => Instruction::North,
            'S' => Instruction::South,
            'E' => Instruction::East,
            'W' => Instruction::West,
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            'F' => Instruction::Forward,
            _ => panic!()
        };
        instructions.push((instr, num));
    }

    let mut x = 0isize;
    let mut y = 0isize;
    let mut dir = 0;

    for (instr, num) in instructions {
        match instr {
            Instruction::North => {
                y += num;
            }
            Instruction::South => {
                y -= num;
            }
            Instruction::East => {
                x += num;
            }
            Instruction::West => {
                x -= num;
            }
            Instruction::Left => {
                // println!()
                dir += num;
            }
            Instruction::Right => {
                dir -= num - 360;
            }
            Instruction::Forward => {
                dir = dir % 360;
                match dir {
                    0 => {
                        x += num;
                    }
                    90 => {
                        y += num;
                    }
                    180 => {
                        x -= num;
                    }
                    270 => {
                        y -= num;
                    }
                    _ => {
                        println!("{:?}", dir);
                        panic!();
                    }
                }
            }
        }
    }

    println!("{:?}", x.abs() + y.abs());

    Ok(())
}
