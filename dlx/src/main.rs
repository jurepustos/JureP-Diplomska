mod sudoku;

use libdlx::*;

fn main() {
    let res = dlx_run(vec![vec![0,1,2], vec![3,4,5]], 6);
    println!("{:?}", res);
}

