use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("./input.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "something went wrong reading the file",
    );

    let length = contents.trim().len();

    let mut sum = 0;
    for (i, c) in contents.trim().chars().enumerate() {
        let n = c.to_digit(10).unwrap();

        let semi_symmetric = (i + length / 2) % length;
        if contents
            .chars()
            .nth(semi_symmetric)
            .unwrap()
            .to_digit(10)
            .unwrap() == n
        {
            sum = sum + n
        }
    }

    println!("{}", sum)
}
