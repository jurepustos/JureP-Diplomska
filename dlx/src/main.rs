mod queens;
mod sudoku;

use crate::sudoku::sudoku_dlx_first;
use std::time::Instant;
use std::thread::JoinHandle;
use std::thread::spawn;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use crate::sudoku::Clue;
use crate::sudoku::sudoku_dlx;
use crate::queens::n_queens_dlx_iter;
use crate::queens::n_queens_dlx_first;
use crate::queens::n_queens_dfs;
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

fn queens_spawn_thread(n: usize, tx: &Sender<(usize, Vec<(usize, usize)>)>) -> JoinHandle<()> {
    let thread_dlx_tx = tx.clone();
    spawn(move || {
        let now = Instant::now();
        if let Some(solution) = n_queens_dlx_first(n) {
            thread_dlx_tx.send((n, solution)).unwrap();
            println!("Thread for n = {} finished", n);
            println!("Took {} ms", now.elapsed().as_millis());
        }
    })
}

fn solve_queens_threaded() {
    static NTHREADS: usize = 15;
    
    let (dlx_tx, dlx_rx) = channel();
    let mut thread_handles = Vec::new();

    let mut n_iter = (30..1000).into_iter();
    for _ in 0..NTHREADS {
        let thread = queens_spawn_thread(n_iter.next().unwrap(), &dlx_tx);
        thread_handles.push(thread);
    } 

    let mut i = 0;
    while let Ok((n, _)) = dlx_rx.recv() {
        println!("i = {}", i);
        i += 1;
        println!("n = {}", n);
        println!();
        queens_spawn_thread(n_iter.next().unwrap(), &dlx_tx);
        // print_queens_solution(n, solution);
    }

    for thread in thread_handles {
        thread.join().unwrap();
    }
} 

fn solve_queens() {
    for n in 30..100 {
        println!("n = {}", n);

        let now = Instant::now();
        if let Some(solution) = n_queens_dlx_first(n) {
            // print_queens_solution(n, solution);
            println!("Took {} ms", now.elapsed().as_millis());
            println!();
        }
        // for solution in n_queens_dlx_iter(n) {
        //     print_queens_solution(n, solution);
        //     // println!("{:?}", dlx_solution);
        //     println!();
        // }
    }
}

fn solve_sudoku(clues: &[Clue]) {
    if let Some(solution) = sudoku_dlx_first(clues) {
        println!("{:?}", solution);
        println!();
    }
}

fn main() {
    // solve_sudoku(&[]);
    solve_queens_threaded()
}

