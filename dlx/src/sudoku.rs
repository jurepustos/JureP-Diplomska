use core::panicking::panic;
use std::collections::{BTreeSet, HashMap};
use std::ops::Range;
use itertools::{Itertools, Product};
use libdlx::{dlx, DLXIter};

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct Clue {
    pub row: u8,
    pub column: u8,
    pub number: u8
}

impl Clue {
    // Constructs a clue, but panics if arguments are invalid
    pub fn new(row: u8, column: u8, number: u8) -> Self {
        let clue = Clue {
            row,
            column,
            number
        };

        if !is_valid_clue(clue) {
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

        if is_valid_clue(clue) {
            Some(clue)
        }
        None
    }

    fn is_valid_clue(self) -> bool {
        self.row < 9 && self.column < 9 && self.number != 0 && self.number <= 9
    }
}

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

fn cover_row(map: &mut HashMap<(u8,u8), [bool; 9]>, clue: &Clue) {
    let row = clue.row;
    for col in 0..9 {
        let mut numbers = map.get_mut(&(row, column)).unwrap();
        numbers[&clue.number-1] = false;
    }
}

fn cover_column(map: &mut HashMap<(u8,u8), [bool; 9]>, clue: &Clue) {
    let col = clue.column;
    for row in 0..9 {
        let mut numbers = map.get_mut(&(row, column)).unwrap();
        numbers[&clue.number-1] = false;
    }
}

fn get_block_index(row: u8, column: u8) -> u8 {
    3 * (row / 3) + column / 3
}

fn cover_block(map: &mut HashMap<(u8,u8), [bool; 9]>, clue: &Clue) {
    let block = get_block_index(clue.row, clue.column);
    for i in 0..3 {
        for j in 0..3 {
            let row = i + 3 * (block / 3);
            let column = j + 3 * (block % 3);
            let mut numbers = map.get_mut(&(row, column)).unwrap();
            numbers[&clue.number-1] = false;
        }
    }
}

pub fn sudoku_dlx(clues: &[Clue]) -> DLXIter<Item> {
    let items = make_items();
    let mut map = HashMap::<(u8,u8), [bool; 9]>::new();
    for i in 0..9 {
        for j in 0..9 {
            map.insert((i,j), [true; 9]);
        }
    }

    for clue in clues {
        cover_row(&mut map, clue);
        cover_column(&mut map, clue);
        cover_block(&mut map, clue);
    }

    let mut iterator =
        (0..9).cartesian_product(0..9).cartesian_product(1..=9);
    let mut sets = Vec::new();
    for ((row, column), number) in iterator {
        if map[(row,column)][number] {
            let position_item = Item::Position(PositionItem {
                row,
                column
            });
            let row_item = Item::Row(RowItem {
                index: row,
                number
            });
            let column_item = Item::Column(ColumnItem {
                index: column,
                number
            });
            let block_item = Item::Block(BlockItem {
                index: get_block_index(row, column),
                number
            });
            let set = vec![position_item, row_item,
                                     column_item, block_item];
            sets.push(set);
        }
    }

    dlx(&sets, &items, &vec![])
}

