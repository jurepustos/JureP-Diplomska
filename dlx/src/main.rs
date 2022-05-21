mod queens;
mod sudoku;
mod vertex_cover;

use std::iter::FromIterator;
use std::collections::BTreeMap;
use std::io::BufRead;
use std::io::BufReader;
use crate::queens::n_queens_dlx_iter_mp;
use crate::vertex_cover::vc_dlxc;
use crate::queens::n_queens_dlx_first_mp;
use crate::sudoku::sudoku_dlx_first;
use std::time::Instant;
use std::thread::JoinHandle;
use std::thread::spawn;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::fs;
use std::env;
use crate::sudoku::Clue;
use crate::sudoku::sudoku_dlx;
use crate::queens::n_queens_dlx_iter;
use crate::queens::n_queens_dlx_first;
use crate::queens::n_queens_dfs;
use libdlx::*;
use maplit::*;

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

        // let now = Instant::now();
        // if let Some(solution) = n_queens_dlx_first_mp(n, 15) {
        //     print_queens_solution(n, solution);
        //     println!("Took {} ms", now.elapsed().as_millis());
        //     println!();
        // }
        // else {
        //     println!("No solution");
        // }
        for solution in n_queens_dlx_iter_mp(n, 15) {
            print_queens_solution(n, solution);
            println!();
        }
    }
}

fn solve_queens() {
    for n in 1..100 {
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
    let triangle_graph_edges = btreemap!{
        0 => vec![1,2], 
        1 => vec![0,2], 
        2 => vec![0,1] 
    };
    let cover = vc_dlxc(&triangle_graph_edges, 2);
    println!("solution: {:?}", cover);
    let cover = vc_dlxc(&triangle_graph_edges, 3);
    println!("solution: {:?}", cover);
    
    println!();

    let star_graph_edges = btreemap!{
        0 => vec![1,2,3,4], 
        1 => vec![0], 
        2 => vec![0], 
        3 => vec![0], 
        4 => vec![0]
    };
    let cover = vc_dlxc(&star_graph_edges, 1);
    println!("solution: {:?}", cover);
    let cover = vc_dlxc(&star_graph_edges, 2);
    println!("solution: {:?}", cover);
    let cover = vc_dlxc(&star_graph_edges, 4);
    println!("solution: {:?}", cover);
}

fn solve_vc_dimacs() {
    
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Expected a filename as first argument.");
    let file = fs::File::open(filename).expect("This file does not exist.");
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines().into_iter()
        .map(|line| line.unwrap()
            .to_owned()
            .split(" ")
            .map(|word| word.to_owned())
            .collect::<Vec<_>>())
        .map(|tokens| (tokens[0].clone(), tokens[1].clone()));

    let (vc, ec) = lines_iter.next().unwrap();
    let vertex_count = str::parse::<usize>(&vc[1..]).unwrap();
    let edge_count = str::parse::<usize>(&ec).unwrap();

    let mut edges = Vec::<(usize, usize)>::with_capacity(edge_count);
    for (v1, v2) in lines_iter {
        edges.push((str::parse(&v1).unwrap(), str::parse(&v2).unwrap()));
    }

    let mut graph = BTreeMap::<usize, Vec<usize>>::new();
    for (v1, v2) in edges {
        if graph.contains_key(&v1) {
            graph.get_mut(&v1).unwrap().push(v2);
        }
        else {
            graph.insert(v1, vec![v2]);
        }

        if graph.contains_key(&v2) {
            graph.get_mut(&v2).unwrap().push(v1);
        }
        else {
            graph.insert(v2, vec![v1]);
        }
    }

    if let Some(cover) = vertex_cover::vc_dlxc(&graph, graph.len()) {
        println!("{:?}, {:?}", cover.len(), cover);
    }

    // let mut i = graph.len();
    // while i > 0 {
    //     println!("i = {}", i);
    //     if let Some(cover) = vertex_cover::vc_dlxc(&graph, i) {
    //         println!("{:?}", cover);
    //         i = cover.len() - 1;
    //     }
    //     else {
    //         break
    //     }
    // }

    // for i in (1..=graph.len()).into_iter().rev() {
    //     println!("i = {}", i);
    //     if let Some(cover) = vertex_cover::vc_dlxc(&graph, i) {
    //         println!("{:?}", cover);
    //     }
    // }
}

fn main() {
    // solve_sudoku(&[]);
    // solve_queens_mp();
    // solve_vertex_cover();
    solve_vc_dimacs();
}
