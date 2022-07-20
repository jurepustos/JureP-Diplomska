pub use dlx::*;
pub use dfs::*;

mod dlx {

    use std::time::Duration;
use crate::dlxc::dlxc_iter_mp;
    use crate::dlxc::dlxc_first;
    use crate::dlxc::dlxc_first_mp;
    use crate::dlxc::dlxc_iter;
    use crate::dlxc::Item;
    use libdlx::dlx::*;

    #[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
    pub enum Position {
        Row(usize),
        Column(usize),
        DownDiagonal(usize),
        UpDiagonal(usize)
    }

    fn n_queens_problem(n: usize) -> Vec<Vec<Item<Position, Position, ()>>> {
        let mut all_sets = Vec::new();
        for i in 0..n {
            for j in 0..n {
                let set = vec![
                    Item::Primary(Position::Row(i)),
                    Item::Primary(Position::Column(j)),
                    Item::Secondary(Position::UpDiagonal(i+j)),
                    Item::Secondary(Position::DownDiagonal(n+i-j))
                ];
                all_sets.push(set);
            }
        }
        all_sets
    }

    fn make_primary_items(n: usize) -> Vec<Position> {
        let mut primary_items = Vec::new();
        for i in 0..n {
            primary_items.push(Position::Row(i));
            primary_items.push(Position::Column(i));
        }
        primary_items
    }

    fn make_secondary_items(n: usize) -> Vec<Position> {
        let mut secondary_items = Vec::new();
        for i in 0..2*n {
            secondary_items.push(Position::UpDiagonal(i));
            secondary_items.push(Position::DownDiagonal(i));
        }
        secondary_items
    }

    pub fn dlx_to_solution(dlx_solution: &Vec<Vec<Item<Position, Position, ()>>>) -> Vec<(usize, usize)> {
        let mut solution = Vec::new();
        for option in dlx_solution {
            let mut row = 0;
            let mut column = 0;
            for position in option {
                if let Item::Primary(Position::Row(i)) = position {
                    row = *i;
                }
                else if let Item::Primary(Position::Column(j)) = position {
                    column = *j;
                }
            }
            solution.push((row, column));
        }

        solution
    }

    pub fn n_queens_dlx_iter(n: usize) -> Box<dyn Iterator<Item = Vec<(usize, usize)>>> {
        let problem_sets = n_queens_problem(n);
        let primary_items = make_primary_items(n);
        let secondary_items = make_secondary_items(n);

        let iter = dlxc_iter(problem_sets, primary_items, secondary_items, Vec::new())
            .filter(|(_, sol)| sol.is_some())
            .map(|(_, sol)| dlx_to_solution(&sol.unwrap().0));
        Box::new(iter)
    }

    pub fn n_queens_dlx_first(n : usize, time_limit: Duration) -> Option<Vec<(usize, usize)>> {
        let problem_sets = n_queens_problem(n);
        let primary_items = make_primary_items(n);
        let secondary_items = make_secondary_items(n);
        let solution = dlxc_first(problem_sets, primary_items, secondary_items, Vec::new(), time_limit);

        solution.map(|(sol, _)| dlx_to_solution(&sol))
    }

    pub fn n_queens_dlx_iter_mp(n: usize, thread_count: usize) -> Box<dyn Iterator<Item = Vec<(usize, usize)>>> {
        let problem_sets = n_queens_problem(n);
        let primary_items = make_primary_items(n);
        let secondary_items = make_secondary_items(n);

        let iter = dlxc_iter_mp(problem_sets, primary_items, secondary_items, Vec::new(), thread_count)
            .map(|sol| dlx_to_solution(&sol.0));
        Box::new(iter)
    }

    pub fn n_queens_dlx_first_mp(n: usize, thread_count: usize) -> Option<Vec<(usize, usize)>> {
        let problem_sets = n_queens_problem(n);
        let primary_items = make_primary_items(n);
        let secondary_items = make_secondary_items(n);
        let solution = dlxc_first_mp(problem_sets, primary_items, secondary_items, Vec::new(), thread_count);

        solution.map(|(sol, _)| dlx_to_solution(&sol))
    }
}

mod dfs {
    use std::time::Duration;
use std::time::Instant;

fn conflict(queens: &[(usize, usize)]) -> bool {
        for i in 1..queens.len() {
            for j in 0..i {
                let (rowa, cola) = queens[i];
                let (rowb, colb) = queens[j];
                if rowa == rowb || cola == colb || (rowa as isize - rowb as isize).abs() == (cola as isize - colb as isize).abs() {
                    return true
                }
            }
        }
        false
    }

    pub fn n_queens_dfs(n: usize, time_limit: Duration) -> Option<Vec<Vec<(usize, usize)>>> {
        let start_time = Instant::now();
        let mut solutions = Vec::new();
        let mut stack = vec![Vec::new()];
        while let Some(solution) = stack.pop() {
            if start_time.elapsed() >= time_limit {
                return None
            }
            if conflict(&solution) {
                continue;
            }

            let row = solution.len();
            if row == n {
                solutions.push(solution);
                continue;
            }

            for column in 0..n {
                let queen = (row, column);
                let mut queens = solution.clone();
                queens.push(queen);
                stack.push(queens);
            }
        }
        Some(solutions)
    }

    pub fn n_queens_dfs_first(n: usize, time_limit: Duration) -> Option<Vec<(usize, usize)>> {
        let start_time = Instant::now();
        let mut stack = vec![Vec::new()];
        while let Some(solution) = stack.pop() {
            if start_time.elapsed() >= time_limit {
                break
            }
            if conflict(&solution) {
                continue;
            }

            let row = solution.len();
            if row == n {
                return Some(solution)
            }

            for column in 0..n {
                let queen = (row, column);
                let mut queens = solution.clone();
                queens.push(queen);
                stack.push(queens);
            }
        }
        None
    }
}
