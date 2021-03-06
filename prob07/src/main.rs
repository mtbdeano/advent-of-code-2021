use std::io;
use std::io::BufRead;
use regex::Regex;

fn lines_from_stdio() -> Vec<String> {
    let file = io::stdin();
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(Debug, Copy, Clone)]
struct Pos {
    number: isize,
    called: bool,
}

#[derive(Debug)]
struct Board {
    board: [Pos; 25],
    row_score: [usize; 5],
    col_score: [usize; 5],
}


impl Board {
    #[inline]
    fn default() -> Board {
        Board {
            board: [Pos { number: 0, called: false}; 25],
            row_score: [0; 5],
            col_score: [0; 5],
        }
    }

    fn play(&mut self, n: isize) {
        for row in 0..5 {
            for col in 0..5 {
                if self.board[row * 5 + col].number == n {
                    // println!("found {:?}", n);
                    self.board[row * 5 + col].called = true;
                    self.row_score[row] = self.row_score[row] + 1;
                    self.col_score[col] = self.col_score[col] + 1;
                }
            }
        }
    }

    fn won(&self) -> bool {
        for row in 0..5 {
            if self.row_score[row] == 5 || self.col_score[row] == 5 {
                return true
            }
        }
        false
    }

    fn score(&self, bingo: isize) -> isize {
        let mut uncalled:isize = 0;
        for row in 0..5 {
            for col in 0..5 {
                if !self.board[row * 5 + col].called {
                    uncalled = uncalled + self.board[row * 5 + col].number;
                }
            }
        }
        return bingo * uncalled;
    }
} 

fn main() {
    let mut lines = lines_from_stdio().into_iter();
    let mut boards = Vec::<Board>::default();
    let splitter = Regex::new(r"\s+").unwrap();

    let numbers_called = lines.next().expect("got some numbers").split(',').map(|n| n.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    println!("{:?}", numbers_called);
    let mut line = lines.next();
    println!("{:?}", line);
    line = lines.next();
    while line.is_some() {
        println!("{:?}", line);
        let mut b = Board::default();
        for row in 0..5 {
            let rline = line.expect("a line");
            //println!("rline {:?}", splitter.split(rline.as_str().trim()).map(|s| s.trim()).collect::<Vec<&str>>());
            let nums: Vec<isize> = splitter.split(rline.as_str().trim()).map(|s| s.parse::<isize>().unwrap()).collect();
            println!("nums {:?}", nums);
            for col in 0..5 {
                b.board[row * 5 + col].number = nums[col];
            }
            line = lines.next();
        }
        boards.push(b);
        line = lines.next();
    }

    println!("boards {:?}", boards.len());
    
    for bingo in numbers_called {
        println!("calling {:?}", bingo);
        for b in 0..boards.len() {
            boards[b].play(bingo);
            if boards[b].won() {
                println!("board {:?} wins with score {:?}", b, boards[b].score(bingo));
                return;
            }
        }
    }
}
