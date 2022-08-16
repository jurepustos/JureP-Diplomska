mod queens;
mod sudoku;
mod vertex_cover;

use crate::queens::n_queens_dlx_first_randomized;
use std::path::Display;
use std::fs::read_dir;
use std::fs::metadata;
use std::time::Duration;
use std::collections::BTreeSet;
use std::iter::FromIterator;
use std::collections::BTreeMap;
use std::io::BufRead;
use std::io::BufReader;
use vertex_cover::vc_reduce_dlxc;
use sudoku::sudoku_dlx_first;
use std::time::Instant;
use std::thread::JoinHandle;
use std::thread::spawn;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::fs;
use std::env;
use sudoku::Clue;
use sudoku::sudoku_dlx;
use queens::n_queens_dlx_iter;
use queens::n_queens_dlx_first;
use queens::n_queens_dfs;
use queens::n_queens_dfs_first;
use libdlx::*;
use maplit::*;

static NTHREADS: usize = 14;
static QUEENS_TIME_LIMIT: Duration = Duration::MAX;
static VC_TIME_LIMIT: Duration = Duration::MAX;

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

fn queens_spawn_thread(n: usize, tx: &Sender<(usize, Option<(Vec<(usize, usize)>, Duration)>)>, 
                       func: fn(usize, Duration) -> Option<Vec<(usize, usize)>>) -> JoinHandle<()> {
    let thread_tx = tx.clone();
    spawn(move || {
        let now = Instant::now();
        if let Some(solution) = func(n, QUEENS_TIME_LIMIT) {
            thread_tx.send((n, Some((solution, now.elapsed())))).unwrap();
        }
        else {
            thread_tx.send((n, None)).unwrap();
        }
    })
}

fn queens_message_format(n: usize, message: Option<(Vec<(usize, usize)>, Duration)>) -> String {
    if let Some((_, time_elapsed)) = message {
        String::from(format!("{} {}", n, time_elapsed.as_millis()))
    }
    else {
        String::from(format!("{} -", n))
    }
}

fn solve_queens_threaded(func: fn(usize, Duration) -> Option<Vec<(usize, usize)>>) {
    let (tx, rx) = channel();
    let mut thread_handles = Vec::new();

    let n_iter: &mut dyn Iterator<Item=usize> = &mut (5..=80).step_by(5)
        .into_iter()
        .map(|n| itertools::repeat_n(n, 10))
        .flatten();
    for _ in 0..NTHREADS {
        let thread = queens_spawn_thread(n_iter.next().unwrap(), &tx, func);
        thread_handles.push(thread);
    }

    let mut i = 0;
    while let Ok((n, message)) = rx.recv() {
        println!("{}", queens_message_format(n, message));
        
        i += 1;
        if let Some(n) = n_iter.next() {
            let thread = queens_spawn_thread(n, &tx, func);
            thread_handles.push(thread);
        }

        // break when all calculations have finished
        // 37 = 40 - 4 + 1
        if i == 37 {
            break;
        }
    }

    for thread in thread_handles {
        thread.join().unwrap();
    }
}

fn solve_queens(n: usize, func: fn(usize, Duration) -> Option<Vec<(usize, usize)>>) {
    let now = Instant::now();
    func(n, QUEENS_TIME_LIMIT);
    println!("{} {}", n, now.elapsed().as_millis());
}

fn test_vertex_cover() {
    let triangle_graph_edges = btreemap!{
        0 => vec![1,2].into_iter().collect(), 
        1 => vec![0,2].into_iter().collect(), 
        2 => vec![0,1].into_iter().collect() 
    };
    let cover = vertex_cover::vc_reduce_dlxc(triangle_graph_edges, Duration::from_secs(1));
    println!("solution: {:?}", cover);
    
    println!();

    let star_graph_edges = btreemap!{
        0 => vec![1,2,3,4].into_iter().collect(), 
        1 => vec![0].into_iter().collect(), 
        2 => vec![0].into_iter().collect(), 
        3 => vec![0].into_iter().collect(), 
        4 => vec![0].into_iter().collect()
    };
    let cover = vertex_cover::vc_reduce_dlxc(star_graph_edges, Duration::from_secs(1));
    println!("solution: {:?}", cover);
}

fn read_dimacs_graph(filename: &str) -> (usize, usize, BTreeMap<usize, BTreeSet<usize>>) {
    let file = fs::File::open(filename).expect("The input file does not exist.");
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines().into_iter()
        .map(|line| line.unwrap()
            .to_owned()
            .split(" ")
            .map(|word| word.to_owned())
            .collect::<Vec<_>>())
        .map(|tokens| (tokens[0].clone(), tokens[1].clone()));

    let mut edges = Vec::<(usize, usize)>::new();
    // n has form #nnnn
    let (n, m) = lines_iter.next().unwrap();
    let vertex_count = str::parse(&n[1..]).unwrap();
    let edge_count = str::parse(&m).unwrap();
    for (v1, v2) in lines_iter {
        edges.push((str::parse(&v1).unwrap(), str::parse(&v2).unwrap()));
    }

    let mut graph = BTreeMap::<usize, BTreeSet<usize>>::new();
    for (v1, v2) in edges {
        if !graph.contains_key(&v1) {
            graph.insert(v1, BTreeSet::new());
        }
        graph.get_mut(&v1).unwrap().insert(v2);

        if !graph.contains_key(&v2) {
            graph.insert(v2, BTreeSet::new());
        }
        graph.get_mut(&v2).unwrap().insert(v1);
    }

    (vertex_count, edge_count, graph)
}

fn solve_reduce_vc(filename: &str) {
    let (vertex_count, edge_count, graph) = read_dimacs_graph(filename);

    let start_time = Instant::now();
    if let Some(_) = vertex_cover::vc_reduce_dlxc(graph, VC_TIME_LIMIT) {
        let elapsed = start_time.elapsed();
        println!("{} {} {}", vertex_count, edge_count, elapsed.as_millis());
    }
    else {
        println!("{} {} -", vertex_count, edge_count);
    }
}

fn solve_pure_vc(filename: &str) {
    let (vertex_count, edge_count, graph) = read_dimacs_graph(filename);

    let start_time = Instant::now();
    if let Some(_) = vertex_cover::vc_pure_dlxc(graph, VC_TIME_LIMIT) {
        let elapsed = start_time.elapsed();
        // println!("{:?}, {:?}", cover.len(), cover);
        println!("{} {} {}", vertex_count, edge_count, elapsed.as_millis());
    }
    else {
        println!("{} {} -", vertex_count, edge_count);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem = &args[1];
    if problem == "queens" {
        let algo = &args[2];
        if algo == "dlx" {
            let n: usize = str::parse(&args[3]).unwrap();
            solve_queens(n, n_queens_dlx_first);
        }
        else if algo == "dlx_random" {
            let n: usize = str::parse(&args[3]).unwrap();
            solve_queens(n, n_queens_dlx_first_randomized);
        }
        else if algo == "dlx_mp" {
            solve_queens_threaded(n_queens_dlx_first);
        }
        else if algo == "dfs" {
            let n: usize = str::parse(&args[3]).unwrap();
            solve_queens(n, n_queens_dfs_first);
        }
        else if algo == "dfs_mp" {
            solve_queens_threaded(n_queens_dfs_first);
        }
    }
    else if problem == "vc" {
        let mode = &args[2];
        let filename = &args[3];
        if mode == "pure" {
            solve_pure_vc(filename);
        }
        else if mode == "reduce" {
            solve_reduce_vc(filename);
        }
        
    }
    else {
        test_vertex_cover()
    }
}
