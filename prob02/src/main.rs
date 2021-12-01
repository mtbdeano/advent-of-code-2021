// use std::io;
use std::io;
use std::io::BufRead;

fn lines_from_stdio() -> Vec<String> {
    let file = io::stdin();
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let lines = lines_from_stdio();
    let mut how_many = 0;
    let mut last_sum = isize::MAX;
    let mut window = Vec::new();

    for line in lines {
        let current: isize = line.parse().unwrap();
        if window.len() == 3 {
            window.remove(0);
        }
        window.push(current);
        let mut sum = isize::MAX;
        if window.len() == 3 {
            sum = window.iter().sum();
        }
        if sum > last_sum {
            println!("{:?} increased", window);
            how_many = how_many + 1;
        } else if sum < last_sum {
            println!("{:?} decreased", window);
        }
        last_sum = sum;
    }
    println!("{:?} in total increased", how_many);
}
