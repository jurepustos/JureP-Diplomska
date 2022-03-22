use libdlx::{dlx, DLXIter};

#[derive(Clone,Copy,Eq,Hash,Debug)]
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

pub fn n_queens_dlx(n: usize) -> DLXIter<Position> {
    let problem_sets = n_queens_problem(n);
    let primary_items = make_primary_items(n);
    let secondary_items = make_secondary_items(n);

    dlx(&problem_sets, &primary_items, &secondary_items)
}
