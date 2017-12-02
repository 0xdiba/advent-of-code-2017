use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("./input.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");


    let first_char = contents.chars().nth(0).unwrap().to_digit(10).unwrap();

    let mut sum = 0;
    let mut prev = 0;
    for (i,c) in contents.trim().chars().enumerate() { 
        let n = c.to_digit(10).unwrap();

        if i == 0 {
            prev = n;
            continue;
        }


        if prev == n {
            sum = sum + prev;
        }
        prev = n;
    }
    if prev == first_char {
        sum = sum + prev
    }

    println!("{}", sum)
}
