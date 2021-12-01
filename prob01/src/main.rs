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
    let mut last = isize::MAX;
    let mut how_many = 0;

    for line in lines {
        let current: isize = line.parse().unwrap();
        if current > last {
            println!("{:?} increased", line);
            how_many = how_many + 1;
        } else if current < last {
            println!("{:?} decreased", line);
        }
        last = current;
    }
    println!("{:?} in total increased", how_many);
}
