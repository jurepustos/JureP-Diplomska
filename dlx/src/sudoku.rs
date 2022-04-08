use std::collections::{HashMap};
use std::ops::{Index, IndexMut};

pub use dlx::{sudoku_dlx};
pub use dfs::{sudoku_dfs};

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct Clue {
    row: u8,
    column: u8,
    number: u8
}

impl Clue {
    // Constructs a clue, but panics if arguments are invalid
    pub fn new(row: u8, column: u8, number: u8) -> Self {
        let clue = Clue {
            row,
            column,
            number
        };

        if !clue.is_valid_clue() {
            panic!("An invalid construction of a Clue object was attempted.")
        }

        clue
    }

    // Constructs a clue, but returns None if arguments are invalid
    pub fn try_new(row: u8, column: u8, number: u8) -> Option<Self> {
        let clue = Clue {
            row,
            column,
            number
        };

        if clue.is_valid_clue() {
            Some(clue)
        }
        else {
            None
        }
    }

    fn is_valid_clue(self) -> bool {
        self.row < 9 && self.column < 9 && self.number != 0 && self.number <= 9
    }
}

fn get_block_index(row: u8, column: u8) -> u8 {
    3 * (row / 3) + column / 3
}

struct Grid([[[bool; 9]; 9]; 9]);

impl Index<(u8, u8, u8)> for Grid {
    type Output = bool;

    fn index(&self, (row, column, number): (u8, u8, u8)) -> &Self::Output {
        self.0.index(row as usize).index(column as usize).index((number-1) as usize)
    }
}

impl IndexMut<(u8, u8, u8)> for Grid {
    fn index_mut(&mut self, (row, column, number): (u8, u8, u8)) -> &mut Self::Output {
        self.0.index_mut(row as usize).index_mut(column as usize).index_mut((number-1) as usize)
    }
}

impl Grid {
    fn new() -> Self {
        Grid([[[true; 9]; 9]; 9])
    }

    fn cover_row(&mut self, clue: Clue) {
        let row = clue.row;
        for column in 0..9 {
            self[(row, column, clue.number)] = false;
        }
    }

    fn cover_column(&mut self, clue: Clue) {
        let column = clue.column;
        for row in 0..9 {
            self[(row, column, clue.number)] = false;
        }
    }

    fn cover_block(&mut self, clue: Clue) {
        let block = get_block_index(clue.row, clue.column);
        for i in 0..3 {
            for j in 0..3 {
                let row = i + 3 * (block / 3);
                let column = j + 3 * (block % 3);
                self[(row, column, clue.number)] = false;
            }
        }
    }

    fn cover(&mut self, clue: Clue) {
        self.cover_row(clue);
        self.cover_column(clue);
        self.cover_block(clue);
    }

    fn uncover_row(&mut self, clue: Clue) {
        let row = clue.row;
        for column in 0..9 {
            self[(row, column, clue.number)] = true;
        }
    }

    fn uncover_column(&mut self, clue: Clue) {
        let column = clue.column;
        for row in 0..9 {
            self[(row, column, clue.number)] = true;
        }
    }

    fn uncover_block(&mut self, clue: Clue) {
        let block = get_block_index(clue.row, clue.column);
        for i in 0..3 {
            for j in 0..3 {
                let row = i + 3 * (block / 3);
                let column = j + 3 * (block % 3);
                self[(row, column, clue.number)] = true;
            }
        }
    }

    fn uncover(&mut self, clue: Clue) {
        self.uncover_row(clue);
        self.uncover_column(clue);
        self.uncover_block(clue);
    }
}

fn init_grid(clues: &[Clue]) -> Grid {
    let mut grid = Grid::new();
    for &clue in clues {
        grid.cover(clue);
    }
    grid
}


mod dlx {
    use itertools::iproduct;
    use libdlx::{dlx, dlx_iter, DLXIter};
    use super::{Clue, get_block_index, init_grid};

    #[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
    pub struct PositionItem {
        row: u8,
        column: u8
    }

    #[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
    pub struct RowItem {
        index: u8,
        number: u8
    }

    #[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
    pub struct ColumnItem {
        index: u8,
        number: u8
    }

    #[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
    pub struct BlockItem {
        index: u8,
        number : u8
    }

    #[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
    pub enum Item {
        Position(PositionItem),
        Row(RowItem),
        Column(ColumnItem),
        Block(BlockItem)
    }

    fn make_items() -> Vec<Item> {
        let mut items = Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                items.push(Item::Position(PositionItem {
                    row: i,
                    column: j
                }));
            }
            for n in 1..=9 {
                items.push(Item::Row(RowItem {
                    index: i,
                    number: n
                }));
                items.push(Item::Column(ColumnItem {
                    index: i,
                    number: n
                }));
                items.push(Item::Block(BlockItem {
                    index: i,
                    number: n
                }));
            }
        }
        items
    }

    fn make_option(row: u8, column: u8, number: u8) -> Vec<Item> {
        vec![
            Item::Position(PositionItem {
                row,
                column
            }),
            Item::Row(RowItem {
                index: row,
                number
            }),
            Item::Column(ColumnItem {
                index: column,
                number
            }),
            Item::Block(BlockItem {
                index: get_block_index(row, column),
                number
            })
        ]
    }

    pub fn sudoku_dlx(clues: &[Clue]) -> DLXIter<Item> {
        let items = make_items();
        let grid = init_grid(clues);

        let mut sets = Vec::new();
        for (number, row, column) in iproduct!(1..=9, 0..9, 0..9) {
            if grid[(row, column, number)] {
                sets.push(make_option(row, column, number));
            }
        }

        dlx_iter(sets, items, vec![])
    }
}


mod dfs {
    use super::{Clue, init_grid};

    fn next_clue(clue: Clue) -> Option<Clue> {
        if clue.number < 9 {
            Some(Clue {
                row: clue.row,
                column: clue.column,
                number: clue.number+1
            })
        }
        else if clue.row < 8 {
            Some(Clue {
                row: clue.row+1,
                column: clue.column,
                number: clue.number
            })
        }
        else if clue.column < 8 {
            Some(Clue {
                row: clue.row,
                column: clue.column+1,
                number: clue.number
            })
        }
        else {
            None
        }
    }

    fn neighbor_clues(clue: Clue) -> Vec<Clue> {
        let mut neighbors = Vec::new();
        if clue.row < 8 {
            neighbors.push(Clue {
                row: clue.row+1,
                column: clue.column,
                number: 1
            });
        }
        if clue.column < 8 {
            neighbors.push(Clue {
                row: clue.row,
                column: clue.column+1,
                number: 1
            });
            if clue.row < 8 {
                neighbors.push(Clue {
                    row: clue.row+1,
                    column: clue.column+1,
                    number: 1
                });
            }
        }

        neighbors
    }

    pub fn sudoku_dfs(clues: &[Clue]) -> Vec<Clue> {
        let mut solution = Vec::new();
        let mut grid = init_grid(clues);
        let mut stack = vec![Clue {
            row: 0,
            column: 0,
            number: 1
        }];
        while let Some(clue) = stack.pop() {
            if grid[(clue.row, clue.column, clue.number)] {
                grid.cover(clue);
                solution.push(clue);

                stack.append(&mut neighbor_clues(clue));
            }
            else {
                if let Some(next) = next_clue(clue) {
                    stack.push(next);
                }
                else {
                    grid.uncover(clue);
                }
                solution.pop();
            }
        }

        solution
    }
}

