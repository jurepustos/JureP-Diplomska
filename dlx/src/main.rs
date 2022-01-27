mod sudoku;

use libdlx::*;
use sudoku::*;

fn main() {
    println!("Hello, world!");

    let empty: Vec<Vec<usize>> = vec![];
    let res = dlx(&empty);
}

