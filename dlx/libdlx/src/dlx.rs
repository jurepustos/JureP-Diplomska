use std::collections::VecDeque;
use std::mem::take;


#[derive(Clone,PartialEq,Eq,Debug)]
pub struct DLXTable<T: Eq + Copy + std::fmt::Debug> {
    names: Vec<Option<T>>,
    left_links: Vec<usize>,
    right_links: Vec<usize>,
    lengths: Vec<usize>,
    up_links: Vec<usize>,
    down_links: Vec<usize>,
    header_links: Vec<usize>
}

struct ColumnIter<'a> {
    down_links: &'a [usize],
    column: usize,
    index: usize
}

impl<'a> ColumnIter<'a> {
    fn new<T>(table: &'a DLXTable<T>, column: usize) -> Self
        where T: Eq + Copy + std::fmt::Debug {
        ColumnIter { 
            down_links: &table.down_links, 
            column, 
            index: column 
        }
    }
} 

impl<'a> Iterator for ColumnIter<'a> {    
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let next_index = self.down_links[self.index];
        if next_index != self.column {
            self.index = next_index;
            Some(next_index)
        }
        else {
            None
        }
    }
}

struct RevColumnIter<'a> {
    up_links: &'a [usize],
    column: usize,
    index: usize
}

impl<'a> RevColumnIter<'a> {
    fn new<T>(table: &'a DLXTable<T>, column: usize) -> Self
        where T: Eq + Copy + std::fmt::Debug {
        RevColumnIter { 
            up_links: &table.down_links, 
            column, 
            index: column 
        }
    }
} 

impl<'a> Iterator for RevColumnIter<'a> {    
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let next_index = self.up_links[self.index];
        if next_index != self.column {
            self.index = next_index;
            Some(next_index)
        }
        else {
            None
        }
    }
}

struct RowIter<'a> {
    header_links: &'a [usize],
    up_links: &'a [usize],
    row_node: usize,
    index: usize
}

impl<'a> RowIter<'a> {
    fn new<T>(table: &'a DLXTable<T>, row_node: usize) -> Self 
        where T: Eq + Copy + std::fmt::Debug {
        RowIter { 
            header_links: &table.header_links, 
            up_links: &table.up_links,
            row_node, 
            index: row_node 
        }
    }
}

impl<'a> Iterator for RowIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let header = self.header_links[self.index];       
        let next_index = 
            if header == 0 {
                self.up_links[self.index]
            }
            else {
                self.index + 1
            };
        if next_index != self.row_node {
            Some(next_index)
        }
        else {
            None
        }
    }
}

struct RevRowIter<'a> {
    header_links: &'a [usize],
    down_links: &'a [usize],
    row_node: usize,
    index: usize
}

impl<'a> RevRowIter<'a> {
    fn new<T>(table: &'a DLXTable<T>, row_node: usize) -> Self 
        where T: Eq + Copy + std::fmt::Debug{
        RevRowIter { 
            header_links: &table.header_links,
            down_links: &table.down_links, 
            row_node, 
            index: row_node 
        }
    }
}

impl<'a> Iterator for RevRowIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let header = self.header_links[self.index];       
        let next_index = 
            if header == 0 {
                self.down_links[self.index]
            }
            else {
                self.index - 1
            };
        if next_index != self.row_node {
            Some(next_index)
        }
        else {
            None
        }
    }
}


impl<T: Eq + Copy + std::fmt::Debug> DLXTable<T> {
    pub fn new(sets: Vec<Vec<T>>, primary_items: Vec<T>, secondary_items: Vec<T>) -> Self {
        let primary_count = primary_items.len();
        let mut names = Vec::with_capacity(1 + primary_items.len() + secondary_items.len());
        names.push(None);
        for item in primary_items {
            names.push(Some(item));
        }

        for item in secondary_items {
            names.push(Some(item));
        }

        let names_count = names.len();

        let node_count = 1 + names_count + sets.len() + sets
            .iter()
            .map(|set| set
                .iter()
                .map(|_| 1)
                .sum::<usize>())
            .sum::<usize>();

        let mut table = DLXTable {
            names,
            left_links: vec![0; names_count],
            right_links: vec![0; names_count],
            lengths: vec![0; names_count],
            up_links: vec![0; node_count],
            down_links: vec![0; node_count],
            header_links: vec![0; node_count]
        };

        // header setup
        table.left_links[0] = names_count - 1;
        table.right_links[0] = primary_count;
        for i in 0..names_count - 1 {
            table.left_links[i+1] = i;
            table.right_links[i] = i+1;
            table.up_links[i+1] = i+1;
            table.down_links[i+1] = i+1;
        }

        let mut prev_spacer = names_count;
        
        let mut current_index = names_count + 1;
        for set in sets {
            if set.len() > 0 {
                for item in set {
                    let header_index = table.names
                        .iter()
                        .position(|name_opt| name_opt.is_some() && item == name_opt.unwrap())
                        .unwrap();
                    table.lengths[header_index] += 1;
                    
                    // node setup
                    table.up_links[current_index] = table.up_links[header_index];
                    table.down_links[current_index] = header_index;
                    table.header_links[current_index] = header_index;

                    // uplink setup
                    table.down_links[table.up_links[current_index]] = current_index;
                    
                    // header setup
                    if table.down_links[header_index] == header_index {
                        table.down_links[header_index] = current_index;
                    }
                    table.up_links[header_index] = current_index;
                    
                    current_index += 1;
                }
    
                // spacer
                table.up_links[current_index] = prev_spacer + 1;
                table.down_links[prev_spacer] = current_index - 1;
                prev_spacer = current_index;
                current_index += 1;
            }
        }

        table
    }

    fn cover(&mut self, column: usize) {
        self.left_links[self.right_links[column]] = self.left_links[column];
        self.right_links[self.left_links[column]] = self.right_links[column];

        let mut i = self.down_links[column];
        while i != column {
            self.hide(i);
            i = self.down_links[i];
        }
    }

    fn uncover(&mut self, column: usize) {
        let mut i = self.up_links[column];
        while i != column {
            self.unhide(i);
            i = self.up_links[i];
        }

        self.left_links[self.right_links[column]] = column;
        self.right_links[self.left_links[column]] = column;
    }
    
    fn hide(&mut self, row_node: usize) {
        let mut i = row_node + 1;
        while i != row_node {
            let header = self.header_links[i];
            // spacer
            if header == 0 {
                i = self.up_links[i];
            }
            else {
                self.up_links[self.down_links[i]] = self.up_links[i];
                self.down_links[self.up_links[i]] = self.down_links[i];
                self.lengths[header] -= 1;

                i += 1;
            }
        }
    }

    fn unhide(&mut self, row_node: usize) {
        let mut i = row_node - 1;
        while i != row_node {
            let header = self.header_links[i];
            // spacer
            if header == 0 {
                i = self.down_links[i];
            }
            else {
                self.lengths[header] += 1;
                self.up_links[self.down_links[i]] = i;
                self.down_links[self.up_links[i]] = i;

                i -= 1;
            }
        }
    }

    fn cover_row(&mut self, row_node: usize) {
        let mut i = row_node + 1;
        while i != row_node {
            let header = self.header_links[i];
            if header == 0 {
                i = self.up_links[i];
            }
            else {
                self.cover(header);
                i += 1;
            }
        }
    }
    
    fn uncover_row(&mut self, row_node: usize) {
        let mut i = row_node - 1;
        while i != row_node {
            let header = self.header_links[i];
            if header == 0 {
                i = self.down_links[i];
            }
            else {
                self.uncover(header);
                i -= 1;
            }
        }
    }

    fn get_row(&self, row_node: usize) -> Vec<T> {
        let mut row = vec![self.names[self.header_links[row_node]].unwrap()];
        let mut k = row_node + 1;
        while k != row_node {
            let header = self.header_links[k];
            if header == 0 {
                k = self.up_links[k];
            }
            else {
                let item = self.names[self.header_links[k]].unwrap(); 
                row.push(item);
                k += 1;
            }
        }
    
        row
    }
}

fn choose_column<T>(table: &DLXTable<T>) -> Option<usize> 
where T: Eq + Copy + std::fmt::Debug {
    let mut j = table.right_links[0];
    let mut s = usize::MAX;
    let mut c = None;
    while j != 0 {
        if table.lengths[j] < s {
            c = Some(j);
            s = table.lengths[j];
        }
        j = table.right_links[j];
    }

    c
}

fn search<T: Eq + Copy + std::fmt::Debug>(table: &mut DLXTable<T>, solution: &mut Vec<usize>) -> Option<Vec<usize>> {
    if let Some(column) = choose_column(table) {
        table.cover(column);

        let mut row_node = table.down_links[column];
        while row_node != column {
            table.cover_row(row_node);

            // recursion
            // go to the next level
            solution.push(row_node);
            let res = search(table, solution);
            if let Some(sol) = res {
                return Some(sol)
            }
            solution.pop();
            
            table.uncover_row(row_node);

            row_node = table.down_links[row_node];
        }

        table.uncover(column);
        None
    }
    else {
        Some(take(solution))
    }
}

#[derive(PartialEq,Eq,Clone,Copy,Debug)]

enum State {
    CoveringColumn,
    CoveringRow,
    BacktrackingColumn,
    BacktrackingRow
}

#[derive(PartialEq,Eq,Clone,Copy,Debug)]
struct LevelState {
    column: usize,
    row_node: usize
}

pub struct DLXIter<T: Eq + Copy + std::fmt::Debug> {
    table: DLXTable<T>,
    stack: Vec<LevelState>,
    state: State
}

impl<T: Eq + Copy + std::fmt::Debug> DLXIter<T> {
    pub fn new(sets: Vec<Vec<T>>, primary_items: Vec<T>, secondary_items: Vec<T>) -> Self {
        let mut table = DLXTable::new(sets, primary_items, secondary_items);
        let mut stack = Vec::new();
        let state = State::CoveringRow;
        if let Some(column) = choose_column(&table) {
            let row_node = table.down_links[column];
            stack.push(LevelState {
                column,
                row_node
            });
            table.cover(column);
        }

        DLXIter { table, stack, state }
    }

    fn cover_column(&mut self, column: usize) {
        self.table.cover(column);
        let row_node = self.table.down_links[column];
        self.stack.push(LevelState { column, row_node });

        if row_node == column {
            // the column is empty
            // set up to return to the previous level
            self.state = State::BacktrackingColumn;
        }
        else {
            // cover the first row
            self.state = State::CoveringRow;
        }
    }

    fn get_solution(&self) -> Vec<Vec<T>> {
        self.stack
            .iter()
            .map(|level| level.row_node)
            .map(|i| self.table.get_row(i))
            .collect()
    }

    fn backtrack_row(&mut self) {
        let mut level = self.stack.pop().unwrap();
        self.table.uncover_row(level.row_node);
        level.row_node = self.table.down_links[level.row_node];
        self.stack.push(level);
        if level.row_node == level.column {
            // we tried the last row
            // set up to return to the previous level
            self.state = State::BacktrackingColumn;
        }
        else {
            // cover the next row
            self.state = State::CoveringRow;
        }
    }
}

impl<T: Eq + Copy + std::fmt::Debug> Iterator for DLXIter<T> {
    type Item = Vec<Vec<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            match self.state {
                State::CoveringColumn => {
                    if let Some(column) = choose_column(&self.table) {
                        // cover next column
                        self.cover_column(column);
                    }
                    else {
                        // all columns are covered
                        self.state = State::BacktrackingRow;
                        return Some(self.get_solution())
                    }
                },
                State::CoveringRow => {
                    // cover the current row and set up for the next level 
                    let level = self.stack.last().unwrap();
                    self.table.cover_row(level.row_node);
                    self.state = State::CoveringColumn;
                },
                State::BacktrackingRow => {
                    // uncover the current row and set up to cover the next one
                    self.backtrack_row()
                }
                State::BacktrackingColumn => {
                    // uncover the last covered column
                    // and set up to continue
                    let level = self.stack.pop().unwrap();
                    self.table.uncover(level.column);
                    self.state = State::BacktrackingRow;
                },
            }
        }
        None
    }
}

pub fn dlx_iter<T: Eq + Copy + std::fmt::Debug>(sets: Vec<Vec<T>>, primary_items: Vec<T>, secondary_items: Vec<T>) -> DLXIter<T> {
    DLXIter::new(sets, primary_items, secondary_items)
}

pub fn dlx_first<T>(sets: Vec<Vec<T>>, primary_items: Vec<T>, secondary_items: Vec<T>) -> Option<Vec<Vec<T>>>
where T: Eq + Copy + std::fmt::Debug {

    let mut table = DLXTable::new(sets, primary_items, secondary_items);
    search(&mut table, &mut Vec::new())
        .map(|solution| solution
            .into_iter()
            .map(|node_index| table.get_row(node_index))
            .collect())
}

pub fn dlx_first_mp<T>(sets: Vec<Vec<T>>, primary_items: Vec<T>, 
                       secondary_items: Vec<T>, thread_count: usize) -> Option<Vec<Vec<T>>> 
where T: Eq + Copy + std::fmt::Debug {   
    todo!()
}

pub fn dlx_iter_mp<T>(sets: Vec<Vec<T>>, primary_items: Vec<T>, 
                      secondary_items: Vec<T>, thread_count: usize) -> DLXIter<T> 
where T: Eq + Copy + std::fmt::Debug {   
    todo!()
}
