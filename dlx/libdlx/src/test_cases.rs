use std::cmp::Reverse;
use std::collections::{BinaryHeap, BTreeSet};
use std::ops::Range;
use rand::{Rng, thread_rng};

fn random_permutation(rng: &mut impl Rng, n: usize) -> Vec<usize> {
    let mut permutation: Vec<usize> = (0..n).into_iter().collect();
    let mut i = n-1;
    while i > 0 {
        let selection = rng.gen_range(0..n);
        let val = permutation[i];
        permutation[i] = permutation[selection];
        permutation[selection] = val;
        i -= 1;
    }

    permutation
}


fn generate_set_sizes(rng: &mut impl Rng, n: usize, r: usize) -> Vec<Range<usize>> {
    let mut boundaries = BinaryHeap::with_capacity(r);
    for _ in 0..r {
        let val = 1 + rng.gen_range(1..n-2);
        boundaries.push(Reverse(val));
    }

    let mut ranges = Vec::with_capacity(r+1);
    let mut prev_bound = 0;
    while let Some(Reverse(bound)) = boundaries.pop() {
        if prev_bound != bound {
            ranges.push(prev_bound..bound);
            prev_bound = bound;
        }
    }
    ranges.push(prev_bound..n);
    ranges
}

pub fn generate_exact_cover(rng: &mut impl Rng, n: usize) -> Vec<Vec<usize>> {
    let r: usize = rng.gen_range(0..n-1);

    let ranges = generate_set_sizes(rng, n, r);
    let permutation: Vec<usize> = random_permutation(rng, n);
    let mut sets = Vec::<Vec<usize>>::with_capacity(ranges.len());
    for range in ranges {
        let mut set = Vec::<usize>::with_capacity(range.len());
        for i in range {
            set.push(permutation[i]);
        }
        set.sort();
        sets.push(set);
    }

    sets
}

fn is_permutation(set: &[usize]) -> bool {
    let mut visited = BTreeSet::new();
    for elem in set {
        if visited.contains(elem) {
            return false;
        }
        visited.insert(*elem);
    }
    true
}

fn is_disjoint(sets: &[Vec<usize>]) -> bool {
    let mut visited = BTreeSet::new();
    for set in sets {
        for elem in set {
            if visited.contains(elem) {
                return false;
            }
            visited.insert(*elem);
        }
    }
    true
}


