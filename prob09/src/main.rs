use std::io;
use std::io::BufRead;
// use regex::Regex;

fn lines_from_stdio() -> Vec<String> {
    let file = io::stdin();
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(Debug,Copy,Clone,Default)]
struct Point {
    x: i64,
    y: i64
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    fn from_str(s: &str) -> Point {
        let nums = s.split(',').map(|n| n.parse::<isize>().unwrap()).collect::<Vec<isize>>();
        Point { x: nums[0] as i64, y: nums[1] as i64}
    }
}

#[derive(Debug)]
struct Seafloor {
    topo: Vec<i64>,
    stride: i64,
    height: i64,
}


impl Seafloor {
    fn default() -> Seafloor {
        Seafloor {
            topo: vec![0; 1000*1000],
            stride: 1000,
            height: 1000
        }
    }

    fn draw(&mut self, from: Point, to: Point) {
        let mut xdelta = to.x - from.x;
        let mut ydelta = to.y - from.y;
        let mut cursor: Point = from;
        // if from.x != to.x && from.y != to.y {
        //     return;
        // }
        if xdelta != 0 {
            xdelta = xdelta / xdelta.abs();
        }
        if ydelta != 0 {
            ydelta = ydelta / ydelta.abs();
        }
        print!("drawing from {:?} to {:?} with xd {:?} yd {:?} ", from, to, xdelta, ydelta);
        while cursor != to {
            let indx = (cursor.x + cursor.y * self.stride) as usize; // silent coerc to unsigned
            self.topo[indx] = self.topo[indx] + 1;
            cursor.x = cursor.x + xdelta;
            cursor.y = cursor.y + ydelta;
            print!(".");
        }
        let indx = (cursor.x + cursor.y * self.stride) as usize; // silent coerc to unsigned
        self.topo[indx] = self.topo[indx] + 1;
        println!();
    }

    fn show(&self, width: usize, height: usize) {
        for y in 0..height {
            for x in 0..width {
                let indx = x + y * self.stride as usize;
                print!("{:03} ", self.topo[indx]);
            }
            println!();
        }
    }

    fn count_overlap(&self, overlap: i64) -> usize {
        let mut count: usize = 0;
        for y in 0..self.height {
            for x in 0..self.stride {
                let indx = x as usize + y as usize * self.stride as usize;
                if self.topo[indx] >= overlap {
                    count = count + 1;
                }
            }
        }        
        count
    }
} 

fn main() {
    let lines = lines_from_stdio();
    let mut sf = Seafloor::default();

    for line in lines {
        let mut split = line.split("->");
        let x = split.next().expect("x").trim();
        let y = split.next().expect("y").trim();
        println!("{:?} {:?}", x, y);
        let from = Point::from_str(x);
        let to = Point::from_str(y);
        
        sf.draw(from, to);
    }

    sf.show(10,10);
    println!("overlap >= 2 {:?} ", sf.count_overlap(2));
}
