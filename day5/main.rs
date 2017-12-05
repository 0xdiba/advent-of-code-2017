
use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() {
    let f = File::open("./input.txt").expect("file not found");

    let mut jumps = Vec::new();
    for line in BufReader::new(f).lines() {
        let n = line.unwrap().parse::<i32>().unwrap();
        jumps.push(n);
    }

    let mut pos = 0; 
    let mut no_jumps = 0;
    loop {
        no_jumps += 1;
        let next = jumps[pos];
        if next >= 3 {
            jumps[pos] = jumps[pos] - 1; 
        } else {
            jumps[pos] = jumps[pos] + 1; 
        };
        pos = (pos as i32 + next) as usize;

        if pos >= jumps.len() {
            break;
        }
    }
    println!("Jumps: {}", no_jumps)
}
