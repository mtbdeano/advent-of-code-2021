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

fn calc_common(lm: String, num_bits: usize, lines: &Vec<String>, pos: usize) -> isize {
    //println!("\npos {:?} lines {:?}", pos, lines);
    if lines.len() == 1 {
        return isize::from_str_radix(lines[0].as_str(), 2).unwrap();
    }

    let mut bit_count = vec![0; num_bits];

    for line in lines {
        // println!("line {:?}", line);
        for (i, b) in line.chars().enumerate() {
            if b == '1' { bit_count[i] = bit_count[i] + 1; }
        }
        
    }

    let just_ones = lines.clone().into_iter().filter(|l| l.chars().nth(pos).unwrap() == '1').collect::<Vec<String>>();
    let just_zeros = lines.clone().into_iter().filter(|l| l.chars().nth(pos).unwrap() == '0').collect::<Vec<String>>();
    //println!("which {:?} ones {:?} zeros {:?}", lm, just_ones.len(), just_zeros.len());

    if bit_count[pos] >= ((lines.len() + 1)/ 2) {
        //println!("ones most likely {:?} at {:?} {:?}", bit_count[pos], pos, lines);
        // the 1's are most likely
        match lm.as_str() {
            "most" => return calc_common(lm, num_bits, &just_ones, pos + 1),
            "least" => return calc_common(lm, num_bits, &just_zeros, pos + 1),
            _ => return -1,
        }
        
    // } else if bit_count[pos] == (lines.len() / 2) {
    //     println!("even at {:?} {:?}", pos, lines);
    //     match lm.as_str() {
    //         "most" => return calc_common(lm, num_bits, &just_ones, pos + 1),
    //         "least" => return calc_common(lm, num_bits, &just_zeros, pos + 1),
    //         _ => return -1,
    //     }        
    } else {
        // the zeros are most likely
        //println!("zeros most likely {:?} at {:?} {:?}", (lines.len() / 2)  - bit_count[pos], pos, lines);
        match lm.as_str() {
            "most" => return calc_common(lm, num_bits, &just_zeros, pos + 1),
            "least" => return calc_common(lm, num_bits, &just_ones, pos + 1),
            _ => return -1,
        }
    }

}



fn main() {
    let lines = lines_from_stdio();

    let first_line = lines[0].clone();
    let num_bits = first_line.len();
    println!("will manage {:?} bits", num_bits);

    let oxygen = calc_common("most".to_string(), num_bits, &lines, 0);
    let co2 = calc_common("least".to_string(), num_bits, &lines, 0);
    
    println!("oxygen {:?} {:b} co2 {:?} {:b}", oxygen, oxygen, co2, co2);
}
