mod queens;
mod bin_packing;
mod sudoku;
mod knapsack;

use crate::queens::dlx_to_solution;
use crate::queens::n_queens_dlx_iter;
use crate::queens::n_queens_dlx;
use crate::queens::n_queens_dfs;
use rand::{Rng, thread_rng};
use libdlx::*;

fn print_solution(n: usize, solution: Vec<(usize, usize)>) {
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

fn main() {
    // for n in 1..=10 {
    //     // for solution in n_queens_dfs(n) {
    //     //     print_solution(n, solution);
    //     //     println!();    
    //     // }
    //     for solution in n_queens_dlx(n) {
    //         print_solution(n, solution);
    //         // println!("{:?}", dlx_solution);
    //         println!();
    //     }
    // }
    
    for n in 0..=13 {
        println!("n = {}", n);
        for solution in n_queens_dlx(n) {
            print_solution(n, solution);
            // println!("{:?}", dlx_solution);
            println!();
        }

    }
    // println!();
    // for solution in n_queens_dlx2(n) {
    //     // print_solution(n, solution);
    //     // println!("{:?}", dlx_solution);
    //     println!();
    // }
    // for solution in n_queens_dfs(n) {
    //     print_solution(n, solution);
    //     println!();    
    // }
}

