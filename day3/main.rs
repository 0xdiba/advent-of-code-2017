use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = args[1].parse::<i32>().unwrap();

    let mut direction = -1;
    let mut count = 1;
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    //start of the spiral
    let mut n: i32 = 1;

    loop {
        if n > input {panic!("Not found!");}

        for _ in 0..count {
            n += 1;
            x += direction;
            if input == n {
                println!("{}", x.abs()+y.abs());
                return;
            }
        }
        for _ in 0..count {
            n += 1;
            y += direction;
            if input == n {
                println!("{}", x.abs()+y.abs());
                return;
            }
        }
        direction *= -1;
        count += 1;
    }
}
