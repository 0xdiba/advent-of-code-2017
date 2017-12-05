use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let f = File::open("./input.txt").expect("file not found");

    let mut chksum = 0;
    for line in BufReader::new(f).lines() {
        chksum += get_chksum(line.unwrap());
    }

    println!("{}", chksum);
}

fn get_chksum(line: String) -> i32 {
    for (i, num) in line.split_whitespace().enumerate() {
        let n = num.parse::<i32>().unwrap();
        for (j, num2) in line.split_whitespace().enumerate() {
            if i == j {
                continue;
            }
            let n2 = num2.parse::<i32>().unwrap();
            let div = n / n2;
            let rem = n % n2;
            if rem != 0 {
                continue;
            }

            return div;
        }
    }
    panic!("No divisible numbers!");
}
