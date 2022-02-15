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
    won: bool,
}

#[inline]
fn rc(r:usize, c:usize) -> usize {
    // internal indexer
    r * 5 + c
}

impl Board {
    #[inline]
    fn default() -> Board {
        Board {
            board: [Pos { number: 0, called: false}; 25],
            row_score: [0; 5],
            col_score: [0; 5],
            won: false,
        }
    }



    fn play(&mut self, n: isize) {
        if !self.won {
            for row in 0..5 {
                for col in 0..5 {
                    if self.board[rc(row, col)].number == n {
                        // println!("found {:?}", n);
                        self.board[rc(row, col)].called = true;
                        self.row_score[row] = self.row_score[row] + 1;
                        self.col_score[col] = self.col_score[col] + 1;
                    }
                }
            }
            for row in 0..5 {
                if self.row_score[row] == 5 || self.col_score[row] == 5 {
                    self.won = true;
                }
            }
        }
    }

    fn score(&self, bingo: isize) -> isize {
        let mut uncalled:isize = 0;
        for row in 0..5 {
            for col in 0..5 {
                if !self.board[rc(row, col)].called {
                    uncalled = uncalled + self.board[rc(row, col)].number;
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
                b.board[rc(row, col)].number = nums[col];
            }
            line = lines.next();
        }
        boards.push(b);
        line = lines.next();
    }

    println!("boards {:?}", boards.len());
    let mut last_won: usize = 0;
    let mut last_score:isize = -1;

    'outer: for bingo in numbers_called {
        println!("calling {:?}", bingo);
        for b in 0..boards.len() {
            boards[b].play(bingo);
            if boards[b].won {
                println!("board {:?} wins with score {:?}", b, boards[b].score(bingo));
                last_won = b;
                last_score = boards[b].score(bingo);
                if boards.iter().all(|b| b.won) {
                    break 'outer
                }
            }
        }

    }
    println!("board {:?} last wins with score {:?}", last_won, last_score);

}
