use std::mem::take;

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum Item<P, S, C> where
    P: Eq + Copy + std::fmt::Debug,
    S: Eq + Copy + std::fmt::Debug,
    C: Eq + Copy + std::fmt::Debug { 
    Primary(P),
    Secondary(S),
    ColoredSecondary(S, C),
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct DLXCTable<P, S, C> where 
    P: Eq + Copy + std::fmt::Debug,
    S: Eq + Copy + std::fmt::Debug,
    C: Eq + Copy + std::fmt::Debug  {
    names: Vec<Option<Item<P, S, C>>>,
    color_names: Vec<Option<C>>,
    left_links: Vec<usize>,
    right_links: Vec<usize>,
    lengths: Vec<usize>,
    up_links: Vec<usize>,
    down_links: Vec<usize>,
    header_links: Vec<usize>,
    colors: Vec<usize>
}


fn add_node<P, S, C>(table: &mut DLXCTable<P, S, C>, current_index: usize, item: Item<P, S, C>) 
    where
    P: Eq + Copy + std::fmt::Debug,
    S: Eq + Copy + std::fmt::Debug,
    C: Eq + Copy + std::fmt::Debug {
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

    if let Item::ColoredSecondary(_, c) = item {
        let color_index = table.color_names
            .iter()
            .position(|color| color.is_some() && c == color.unwrap())
            .unwrap();
        table.colors[current_index] = table.colors[color_index];
    }
}

impl<P, S, C> DLXCTable<P, S, C> where
    P: Eq + Copy + std::fmt::Debug,
    S: Eq + Copy + std::fmt::Debug,
    C: Eq + Copy + std::fmt::Debug {
    pub fn new(sets: Vec<Vec<Item<P, S, C>>>, primary_items: Vec<P>, secondary_items: Vec<S>, colors: Vec<C>) -> Self {
        let primary_count = primary_items.len();
        let mut names = Vec::with_capacity(1 + primary_items.len() + secondary_items.len());
        names.push(None);
        for item in primary_items {
            names.push(Some(Item::Primary(item)));
        }

        for item in secondary_items {
            names.push(Some(Item::Secondary(item)));
        }

        let names_count = names.len();
        
        
        let mut color_names = vec![None; colors.len() + 1];
        for color in colors {
            color_names.push(Some(color));
        }

        let node_count = 1 + names_count + sets.len() + sets
            .iter()
            .map(|set| set
                .iter()
                .map(|_| 1)
                .sum::<usize>())
            .sum::<usize>();

        let mut table = DLXCTable {
            names,
            color_names,
            left_links: vec![0; names_count],
            right_links: vec![0; names_count],
            lengths: vec![0; names_count],
            up_links: vec![0; node_count],
            down_links: vec![0; node_count],
            header_links: vec![0; node_count],
            colors: vec![0; node_count]
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
                    add_node(&mut table, current_index, item);
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

    fn commit(&mut self, row_node: usize) {
        if self.colors[row_node] == 0 {
            let header = self.header_links[row_node];
            self.cover(header);
        }
        else {
            self.purify(row_node);
        }
    }

    fn uncommit(&mut self, row_node: usize) {
        if self.colors[row_node] == 0 {
            let header = self.header_links[row_node];
            self.uncover(header);
        }
        else {
            self.unpurify(row_node);
        }
    }

    fn purify(&mut self, row_node: usize) {
        let color = self.colors[row_node];
        let header = self.header_links[row_node];
        
        let mut i = self.down_links[header];
        while i != header {
            if self.colors[i] == color {
                self.colors[i] = usize::MAX;
            }
            else {
                self.hide(i);
            }

            i = self.down_links[i];
        }
    }

    fn unpurify(&mut self, row_node: usize) {
        let color = self.colors[row_node];
        let header = self.header_links[row_node];
        
        let mut i = self.up_links[header];
        while i != header {
            if self.colors[i] == usize::MAX {
                self.colors[i] = color;
            }
            else {
                self.unhide(i);
            }

            i = self.up_links[i];
        }
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
            if self.colors[i] != usize::MAX {
                let header = self.header_links[i];
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
    }

    fn unhide(&mut self, row_node: usize) {
        let mut i = row_node - 1;
        while i != row_node {
            if self.colors[i] != usize::MAX {
                let header = self.header_links[i];
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
    }

    fn cover_row(&mut self, row_node: usize) {
        let mut i = row_node + 1;
        while i != row_node {
            let header = self.header_links[i];
            if header == 0 {
                i = self.up_links[i];
            }
            else {
                self.commit(i);
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
                self.uncommit(i);
                i -= 1;
            }
        }
    }

    fn get_row(&self, row_node: usize) -> Vec<Item<P, S, C>> {
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

fn choose_column<P, S, C>(table: &DLXCTable<P, S, C>) -> Option<usize> 
    where
    P: Eq + Copy + std::fmt::Debug,
    S: Eq + Copy + std::fmt::Debug,
    C: Eq + Copy + std::fmt::Debug {
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

fn search<P, S, C>(table: &mut DLXCTable<P, S, C>, solution: &mut Vec<usize>) -> Option<Vec<usize>>
    where
    P: Eq + Copy + std::fmt::Debug,
    S: Eq + Copy + std::fmt::Debug,
    C: Eq + Copy + std::fmt::Debug {
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

pub struct DLXIter<P, S, C> where
    P: Eq + Copy + std::fmt::Debug,
    S: Eq + Copy + std::fmt::Debug,
    C: Eq + Copy + std::fmt::Debug {
    table: DLXCTable<P, S, C>,
    stack: Vec<LevelState>,
    state: State
}

impl<P, S, C> DLXIter<P, S, C>
    where
    P: Eq + Copy + std::fmt::Debug,
    S: Eq + Copy + std::fmt::Debug,
    C: Eq + Copy + std::fmt::Debug {
    pub fn new(sets: Vec<Vec<Item<P, S, C>>>, primary_items: Vec<P>, secondary_items: Vec<S>, colors: Vec<C>) -> Self {
        let mut table = DLXCTable::new(sets, primary_items, secondary_items, colors);
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

    fn get_solution(&self) -> Vec<Vec<Item<P, S, C>>> {
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

impl<P, S, C> Iterator for DLXIter<P, S, C> where
    P: Eq + Copy + std::fmt::Debug,
    S: Eq + Copy + std::fmt::Debug,
    C: Eq + Copy + std::fmt::Debug {
    type Item = Vec<Vec<Item<P, S, C>>>;

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

pub fn dlx_iter<P, S, C>(sets: Vec<Vec<Item<P, S, C>>>, primary_items: Vec<P>, secondary_items: Vec<S>, colors: Vec<C>) -> DLXIter<P, S, C>
    where
    P: Eq + Copy + std::fmt::Debug,
    S: Eq + Copy + std::fmt::Debug,
    C: Eq + Copy + std::fmt::Debug {
    DLXIter::new(sets, primary_items, secondary_items, colors)
}

pub fn dlx_first<P, S, C>(sets: Vec<Vec<Item<P, S, C>>>, primary_items: Vec<P>, secondary_items: Vec<S>, colors: Vec<C>) -> Option<Vec<Vec<Item<P, S, C>>>>
    where 
    P: Eq + Copy + std::fmt::Debug,
    S: Eq + Copy + std::fmt::Debug,
    C: Eq + Copy + std::fmt::Debug  {
    let mut table = DLXCTable::new(sets, primary_items, secondary_items, colors);
    search(&mut table, &mut Vec::new())
        .map(|solution| solution
            .into_iter()
            .map(|node_index| table.get_row(node_index))
            .collect())
}
