#![allow(unused_variables)]
#![allow(dead_code)]
use std::string::ParseError;
use std::{path::Path, fs::File, io::Read};
use std::str::FromStr;

fn load_file(loc : &str) -> String {
    let path = Path::new(loc);
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(e) => panic!("Could not load file {}: {}", loc, e),
        Ok(_) => s,
    }
}

fn main() {
    day_2();
}

fn day_1() {
    print!("Day 1\n");
    let input = load_file("../input1");

    // Might be better with a inplace sort

    let mut blocks : Vec<i32> = input
        .split("\n\r") // Should be just \n\n but might be a encoding/OS issue
        .map(|block_string | {
            block_string
                .split("\n")
                .filter(|e|e.len() > 0)
                .map(|v| match v.trim().parse() {
                    Err(e) => panic!("i32 parse error: {:?} : {}", v, e),
                    Ok(r) => r,
                })
                .collect::<Vec<i32>>()
                .iter()
                .sum()
        })
        .collect();
    blocks.sort_by(|a,b|b.cmp(a));
    println!("Stage 1: {}", blocks[0]);
    println!("Stage 2: {}", blocks[0]+blocks[1]+blocks[2]);
}

#[derive(Debug)]
struct Game_Result {
    value: usize,
}

impl FromStr for Game_Result {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        const WIN_LOSE: [usize; 3] = [3,6,0];
        fn to_score(input: &str) -> usize {
            match input {
                "A" | "X" => 1, // Rock
                "B" | "Y" => 2, // Paper
                "C" | "Z" => 3, // Scissor
                _ => unreachable!("wow"),
            }
        }
        let (p1, p2) = match s.split_once(" ") {
            Some((p1,p2)) => (p1,p2),
            None => panic!("ERROR"),
        };
        let p1 = to_score(p1);
        let p2 = to_score(p2);
        let score = p1 + WIN_LOSE[
            (2 + p1 + p2) % WIN_LOSE.len()
        ];
        return Ok(Game_Result { value: score });
    }
}


fn day_2() {
    println!("Day 2");
    // 2 + A(0)
    // 2 + B(1)
    // 2 + C(2)
    let values : Vec<Game_Result> = include_str!("../../input2")
        .trim()
        .lines()
        .flat_map(|e| e.parse())
        .collect();
    println!("{:?}", values)
}
