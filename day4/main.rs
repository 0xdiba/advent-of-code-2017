use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() {
    let f = File::open("./input.txt").expect("file not found");

    let mut valid = 0;
    for line in BufReader::new(f).lines() {
        if is_valid(line.unwrap()) {
            valid += 1;
        } 
    }

    println!("{}",valid);
}

fn is_valid(line: String) -> bool {
    let passwds = line.split_whitespace();
    let size = line.split_whitespace().count();
    for (i, pass) in passwds.enumerate() {
        for j in i+1..size{
            if pass == line.split_whitespace().nth(j).unwrap(){
                return false;
            }
        }
    }
    return true;
}
