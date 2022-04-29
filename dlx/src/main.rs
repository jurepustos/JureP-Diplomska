mod queens;
mod sudoku;
mod vertex_cover;

use crate::vertex_cover::vc_dlxc;
use crate::queens::n_queens_dlx_first_mp;
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

fn solve_queens_mp() {
    for n in 1..1000 {
        println!("n = {}", n);

        let now = Instant::now();
        if let Some(solution) = n_queens_dlx_first_mp(n, 15) {
            print_queens_solution(n, solution);
            println!("Took {} ms", now.elapsed().as_millis());
            println!();
        }
        else {
            println!("No solution");
        }
    }
}

fn solve_queens() {
    for n in 80..100 {
        println!("n = {}", n);

        let now = Instant::now();
        if let Some(solution) = n_queens_dlx_first(n) {
            print_queens_solution(n, solution);
            println!("Took {} ms", now.elapsed().as_millis());
            println!();
        }
        else {
            println!("No solution");
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

fn solve_vertex_cover() {
    let triangle_graph_edges = vec![(0,1), (1,2), (2,0)];
    let cover = vc_dlxc(&triangle_graph_edges, 2);
    println!("solution: {:?}", cover);
    let cover = vc_dlxc(&triangle_graph_edges, 5);
    println!("solution: {:?}", cover);
    
    println!();

    let star_graph_edges = vec![(0,1), (0,2), (0,3), (0,4)];
    let cover = vc_dlxc(&star_graph_edges, 1);
    println!("solution: {:?}", cover);
    let cover = vc_dlxc(&star_graph_edges, 2);
    println!("solution: {:?}", cover);
}

fn main() {
    // solve_sudoku(&[]);
    solve_queens_mp();
    // solve_vertex_cover();
}
