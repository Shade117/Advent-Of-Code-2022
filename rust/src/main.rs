use std::{path::Path, fs::File, io::Read};

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
    day_1();
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
                .map(|v| match v.trim().parse() { // Trim shouldn't be required
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