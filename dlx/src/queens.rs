pub use dlx::*;
pub use dfs::*;

mod dlx {

    use libdlx::{dlx, dlx_iter, DLXIter};

    #[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
    pub enum Position {
        Row(usize),
        Column(usize),
        DownDiagonal(usize),
        UpDiagonal(usize)
    }

    fn n_queens_problem(n: usize) -> Vec<Vec<Position>> {
        let mut all_sets = Vec::new();
        for i in 0..n {
            for j in 0..n {
                let set = vec![
                    Position::Row(i),
                    Position::Column(j),
                    Position::UpDiagonal(i+j),
                    Position::DownDiagonal(n+i-j)
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

    pub fn dlx_to_solution(dlx_solution: &Vec<Vec<Position>>) -> Vec<(usize, usize)> {
        let mut solution = Vec::new();
        for option in dlx_solution {
            let mut row = 0;
            let mut column = 0;
            for position in option {
                if let Position::Row(i) = position {
                    row = *i;
                }
                else if let Position::Column(j) = position {
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

        let iter = dlx_iter(problem_sets, primary_items, secondary_items)
            .map(|sol| dlx_to_solution(&sol));
        Box::new(iter)
    }

    pub fn n_queens_dlx(n : usize) -> Vec<Vec<(usize, usize)>> {
        let problem_sets = n_queens_problem(n);
        let primary_items = make_primary_items(n);
        let secondary_items = make_secondary_items(n);
        let solutions = dlx(problem_sets, primary_items, secondary_items);

        solutions
            .into_iter()
            .map(|solution| dlx_to_solution(&solution))
            .collect()
    }
}

mod dfs {
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

    pub fn n_queens_dfs(n: usize) -> Vec<Vec<(usize, usize)>> {
        let mut solutions = Vec::new();
        let mut stack = vec![Vec::new()];
        while let Some(solution) = stack.pop() {
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
        solutions
    }
}
