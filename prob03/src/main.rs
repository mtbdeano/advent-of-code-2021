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
    let mut depth = 0;
    let mut position = 0;


    for line in lines {
        let mut split = line.split(" ");
        let first_word = split.next();
        let second_word = split.next();

        if second_word.is_none() { continue; }
    
        let distance : isize = second_word.unwrap().parse().unwrap();
        //println!("direction {:?} for {:?}", instruction, distance);
        match first_word {
            Some("forward") => {
                position += distance;
            },
            Some("backward") => {
                position -= distance;
            },
            Some("up") => {
                depth -= distance;
            },
            Some("down") => {
                depth += distance;
            },
            Some(&_) | None => {},
        }
        println!("distance {:?} depth {:?}", position, depth);
    }
    println!("product is {:?}", position * depth);
}
