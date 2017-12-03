use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() {
    let f = File::open("./input.txt").expect("file not found");

    let mut chksum = 0;
    for line in BufReader::new(f).lines() {
        chksum += get_chksum(line.unwrap()); 
        //println!("================= {}", get_chksum(line.unwrap()));
    }

    println!("{}",chksum);
}

fn get_chksum(line: String) -> i32 {
    let mut max = std::i32::MIN;
    let mut min = std::i32::MAX;
    for num in line.split_whitespace() {
        let n = num.parse::<i32>().unwrap();
        if n > max {
            max = n;
        }

        if n < min {
            min = n;
        }
    }

    return max-min;
}
