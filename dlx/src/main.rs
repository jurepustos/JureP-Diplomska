mod sudoku;

use rand::{Rng, thread_rng};
use libdlx::*;

fn main() {
    for i in 0..10_usize.pow(5) {
        let mut sets = Vec::with_capacity(20);
        let mut rng = thread_rng();
        let n: usize = rng.gen_range(5..10_usize.pow(3));
        for _ in 0..20 {
            let case = generate_exact_cover(&mut rng, n);
            for set in case {
                if !sets.contains(&set) {
                    sets.push(set);
                }
            }
        }
        println!("Sets {:?}", sets);
        let cover = dlx_run(sets, n);
        println!("Covers {:?}", cover);
    }
}

