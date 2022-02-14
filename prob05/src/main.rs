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

    let first_line = lines[0].clone();
    let num_bits = first_line.len();
    println!("will manage {:?} bits", num_bits);
    let mut bit_count = vec![0; num_bits];
    let line_count = lines.len();

    for line in lines {
        // println!("line {:?}", line);
        for (i, b) in line.chars().enumerate() {
            if b == '1' { bit_count[i] = bit_count[i] + 1; }
        }

    }
    let mut gamma : usize = 0;
    let mut epsilon : usize = 0;
    for count in bit_count {
        //println!("count {:?} is {:?}/{:?}", i, count, line_count);
        if count >= (line_count / 2) {
            // this bit is mostly 1
            gamma = gamma << 1 | 1;
            epsilon = epsilon << 1;
        } else {
            // this bit is mostly 0
            gamma = gamma << 1;
            epsilon = epsilon << 1 | 1;
        }
    }
    println!("gamma {:?} {:b} epsilon {:?} {:b}", gamma, gamma, epsilon, epsilon);
    println!("power {:?}", gamma * epsilon)
}
