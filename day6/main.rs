use std::fs::File;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};

fn main() {
    let f = File::open("./input.txt").expect("file not found");

    let mut banks = Vec::new();
    for line in BufReader::new(f).lines() {
        for bank in line.unwrap().split_whitespace() {
            banks.push(bank.parse::<i32>().unwrap());
        }

        let mut states = HashMap::new();
        let len = banks.len();
        let mut steps = 0;
        loop {
            steps += 1;

            let mut index = max_index(banks.as_slice());
            let mut block = banks[index];
            banks[index] = 0;

            loop {
                if block == 0 {
                    break;
                }
                index = (index + 1) % len;

                banks[index] = banks[index] + 1;
                block -= 1;
            }

            let hash = hash(banks.as_slice());
            if states.contains_key(&hash) {
                println!("{}", steps - states.get(&hash).unwrap());
                return;
            } else {
                states.insert(hash, steps);
            }
        }
    }
}

fn hash(v: &[i32]) -> String {
    let mut hash = "".to_owned();
    for i in v {
        let num = i.to_string();
        hash.push_str(&num);
    }

    return hash;
}

fn max_index(array: &[i32]) -> usize {
    let mut i = 0;

    for (j, &value) in array.iter().enumerate() {
        if value > array[i] {
            i = j;
        }
    }

    i
}
