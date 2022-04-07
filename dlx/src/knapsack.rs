use std::mem::take;
use std::cmp::max;

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub struct Item {
    pub value: usize,
    pub volume: usize
}

pub fn knapsack_naive(max_volume: usize, items: &[Item]) -> (Vec<Item>, usize) {
    if items.len() == 0 || max_volume == 0 {
        (Vec::new(), 0)
    }
    else {
        let item = items[0];
        let remaining_items = &items[1..items.len()];
        let (exclude, exc_value) = knapsack_naive(max_volume, remaining_items);
        if item.volume > max_volume {
            let (mut include, inc_value) = knapsack_naive(max_volume - item.volume, remaining_items);
            if inc_value + item.value > exc_value {
                include.push(item);
                return (include, inc_value + item.value)
            }
        }

        (exclude, exc_value)
    }
}

pub fn knapsack_dp(max_volume: usize, items: &[Item]) -> (Vec<usize>, usize) {
    let mut solutions: Vec<Vec<usize>> = vec![Vec::new()];
    let mut dp = vec![vec![(0, 0); items.len()]; max_volume + 1];

    for v in 0..=max_volume {
        for i in 0..items.len() {
            if v == 0 || i == 0 {
                dp[v][i] = (0, 0);
            }
            else if items[i].volume > v {
                dp[v][i] = dp[v][i-1];
            }
            else {
                let item = items[i];
                let (inc_index, inc_value) = dp[v - item.volume][i - 1];
                let (exc_index, exc_value) = dp[v][i - 1];

                if inc_value + items[i].value > exc_value {
                    let include = &solutions[inc_index];
                    let mut new_include = include.clone();
                    new_include.push(i);
                    solutions.push(new_include);
                    dp[v][i] = (solutions.len() - 1, inc_value + item.value);
                }
                else {
                    dp[v][i] = (exc_index, exc_value);
                }
            }
        }
    }

    let (sol_index, volume) = dp[max_volume][items.len() - 1] ;
    let solution = take(&mut solutions[sol_index]);
    (solution, volume)
}

pub fn knapsack_dlx(max_volume: usize, items: &[Item]) -> (Vec<Item>, usize) {
    todo!()
}
