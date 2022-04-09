mod queens;
mod bin_packing;
mod sudoku;
mod knapsack;

use crate::sudoku::sudoku_dlx;
use crate::queens::dlx_to_solution;
use crate::queens::n_queens_dlx_iter;
use crate::queens::n_queens_dlx;
use crate::queens::n_queens_dfs;
use rand::{Rng, thread_rng};
use libdlx::*;

fn print_queens_solution(n: usize, solution: Vec<(usize, usize)>) {
    let mut output = String::from("");
    for row in 0..n {
        for column in 0..n {
            if solution.contains(&(row, column)) {
                output.push('Q');
            }
            else {
                output.push('.');
            }
        }
        output.push('\n');
    }
    println!("{}", output);
}

fn solve_queens() {
    for n in 0..=13 {
        println!("n = {}", n);
        for solution in n_queens_dlx_iter(n) {
            print_queens_solution(n, solution);
            // println!("{:?}", dlx_solution);
            println!();
        }
    }
} 

fn solve_sudoku() {
    for solution in sudoku_dlx(&[]) {
        println!("{:?}", solution);
        println!();
    }
}

fn main() {
    solve_sudoku();
}

