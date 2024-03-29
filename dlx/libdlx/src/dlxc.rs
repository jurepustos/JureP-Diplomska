use std::time::Instant;
use std::time::Duration;
use rand::seq::SliceRandom;

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum Item<P, S, C> 
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug { 
    Primary(P),
    Secondary(S),
    ColoredSecondary(S, C),
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct DLXCTable<P, S, C> 
where 
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
    colors: Vec<usize>,
}

fn has_name<P, S, C>(item: Item<P, S, C>, name: Option<Item<P, S, C>>) -> bool
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    match (name, item) {
        (Some(Item::Primary(n)), Item::Primary(i)) => i == n,
        (Some(Item::Secondary(n)), Item::Secondary(i)) => i == n,
        (Some(Item::Secondary(n)), Item::ColoredSecondary(i, _)) => i == n,
        _ => false
    }
}

fn add_node<P, S, C>(table: &mut DLXCTable<P, S, C>, index: usize, item: Item<P, S, C>) 
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    let header_index = table.names
        .iter()
        .position(|&name| has_name(item, name))
        .expect(&format!("{:?} not present", item));
    table.lengths[header_index] += 1;
    
    // node setup
    table.up_links[index] = table.up_links[header_index];
    table.down_links[index] = header_index;
    table.header_links[index] = header_index;

    // uplink setup
    table.down_links[table.up_links[index]] = index;
    
    // header setup
    if table.down_links[header_index] == header_index {
        table.down_links[header_index] = index;
    }
    table.up_links[header_index] = index;

    if let Item::ColoredSecondary(_, c) = item {
        let color_index = table.color_names
            .iter()
            .position(|color| color.is_some() && c == color.unwrap())
            .unwrap();
        table.colors[index] = color_index;
    }
}

impl<P, S, C> DLXCTable<P, S, C> 
where
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
        
        
        let mut color_names = Vec::with_capacity(colors.len() + 1);
        color_names.push(None);
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
        table.left_links[0] = primary_count;
        for i in 0..primary_count {
            table.left_links[i+1] = i;
            table.right_links[i] = i+1;
            table.up_links[i+1] = i+1;
            table.down_links[i+1] = i+1;
        }

        table.left_links[primary_count + 1] = names_count - 1;

        table.up_links[primary_count + 1] = primary_count + 1;
        table.down_links[primary_count + 1] = primary_count + 1;
        for i in primary_count+1..names_count-1 {
            table.left_links[i+1] = i;
            table.right_links[i] = i+1;
            table.up_links[i+1] = i+1;
            table.down_links[i+1] = i+1;
        }

        table.right_links[names_count - 1] = primary_count + 1;

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
        let color = self.colors[row_node];
        let header = self.header_links[row_node];
        if color == 0 {
            self.cover(header);
        }
        else if color != usize::MAX {
            self.purify(row_node);
            self.colors[header] = color;
        }
    }

    fn uncommit(&mut self, row_node: usize) {
        let color = self.colors[row_node];
        let header = self.header_links[row_node];
        if color == 0 {
            self.uncover(header);
        }
        else if color != usize::MAX {
            self.unpurify(row_node);
            self.colors[header] = 0;
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
                    if self.lengths[header] == 0 {
                        panic!("underflowing header {:?}", self.names[header]);
                    }
                    else {
                        self.lengths[header] -= 1;
                    }
    
                    i += 1;
                }
            }
            else {
                i += 1;
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
            else {
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

    fn get_item(&self, row_node: usize) -> Item<P, S, C> {
        let header = self.header_links[row_node];
        match self.names[header] {
            Some(Item::Primary(item)) => Item::Primary(item),
            Some(Item::Secondary(item)) => {
                if let Some(color) = self.color_names[self.colors[header]] {
                    Item::ColoredSecondary(item, color)
                }
                else {
                    Item::Secondary(item)
                }
            },
            _ => panic!("None or ColoredSecondary in table headers. Something went horribly wrong.")
        }
    }

    fn get_row(&self, row_node: usize) -> Vec<Item<P, S, C>> {
        let mut row = vec![self.get_item(row_node)];
        let mut k = row_node + 1;
        while k != row_node {
            let header = self.header_links[k];
            if header == 0 {
                k = self.up_links[k];
            }
            else {
                let item = self.get_item(k);
                row.push(item);
                k += 1;
            }
        }
    
        row
    }

    fn get_colors(&self) -> Vec<(S, Option<C>)> {
        let mut assignments = Vec::new();
        for (i, name) in self.names.iter().enumerate() {
            if let Some(Item::Secondary(item)) = *name {
                let color = self.color_names[self.colors[i]]; 
                assignments.push((item, color));
            }
        }

        assignments
    }
}

fn min_length_column<P, S, C>(table: &DLXCTable<P, S, C>) -> Option<usize> 
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    let mut i = table.right_links[0];
    let mut size = usize::MAX;
    let mut column = None;
    while i != 0 {
        if table.lengths[i] < size {
            column = Some(i);
            size = table.lengths[i];
        }
        i = table.right_links[i];
    }

    column
}

fn min_length_column_randomized<P, S, C>(table: &DLXCTable<P, S, C>) -> Option<usize>
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    let mut i = table.right_links[0];
    let mut size = usize::MAX;
    let mut columns = Vec::new();
    while i != 0 {
        if table.lengths[i] < size {
            columns = Vec::from([i]);
            size = table.lengths[i];
        }
        else if table.lengths[i] == size {
            columns.push(i);
        }
        i = table.right_links[i];
    }
    columns.choose(&mut rand::thread_rng()).cloned()
}

type Solution<P, S, C> = (Vec<Vec<Item<P, S, C>>>, Vec<(S, Option<C>)>);

fn search<P, S, C>(table: &mut DLXCTable<P, S, C>, choose_column: fn(&DLXCTable<P, S, C>) -> Option<usize>, 
                   partial_solution: &mut Vec<usize>) -> Option<Solution<P, S, C>>
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
            partial_solution.push(row_node);
            let res = search(table, choose_column, partial_solution);
            if let Some((solution, colors)) = res {
                return Some((solution, colors))
            }
            partial_solution.pop();

            
            table.uncover_row(row_node);

            row_node = table.down_links[row_node];
        }

        table.uncover(column);
        None
    }
    else {
        let solution = partial_solution
            .iter()
            .map(|row_node| table.get_row(*row_node))
            .collect();
        
        Some((solution, table.get_colors()))
    }
}

#[derive(PartialEq,Eq,Clone,Copy,Debug)]
pub enum State {
    CoveringColumn,
    CoveringRow,
    BacktrackingColumn,
    BacktrackingRow,
    FoundSolution,
}

#[derive(PartialEq,Eq,Clone,Copy,Debug)]
struct LevelState {
    column: usize,
    row_node: usize
}

pub struct DLXCIter<P, S, C> 
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    table: DLXCTable<P, S, C>,
    stack: Vec<LevelState>,
    state: State,
    choose_column: fn(&DLXCTable<P, S, C>) -> Option<usize>
}

impl<P, S, C> DLXCIter<P, S, C>
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    pub fn new(sets: Vec<Vec<Item<P, S, C>>>, choose_column: fn(&DLXCTable<P, S, C>) -> Option<usize>, 
               primary_items: Vec<P>, secondary_items: Vec<S>, colors: Vec<C>) -> Self {
        let table = DLXCTable::new(sets, primary_items, secondary_items, colors);
        let stack = Vec::new();
        let state = State::CoveringColumn;
        let mut this = DLXCIter { table, stack, state, choose_column };
        this.cover_column();
        this
    }

    fn cover_column(&mut self) {
        if let Some(column) = (self.choose_column)(&self.table) {
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
        else {
            // all columns are covered
            self.state = State::FoundSolution;
        }
    }

    fn cover_row(&mut self) {
        // cover the current row and set up for the next level 
        let level = self.stack.last().unwrap();
        self.table.cover_row(level.row_node);
        self.state = State::CoveringColumn;
    }

    fn backtrack_column(&mut self) {
        // uncover the last covered column
        // and set up to continue
        let level = self.stack.pop().unwrap();
        self.table.uncover(level.column);
        self.state = State::BacktrackingRow;
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

    pub fn get_solution(&self) -> Option<(Vec<Vec<Item<P, S, C>>>, Vec<(S, Option<C>)>)> {
        if let State::FoundSolution = self.state {
            let solution = self.stack
                .iter()
                .map(|level| level.row_node)
                .map(|i| self.table.get_row(i))
                .collect();
            Some((solution, self.table.get_colors()))
        }
        else {
            None
        }
    }
}

impl<P, S, C> DLXCIter<P, S, C> 
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    fn first_solution(&mut self, time_limit: Duration) -> Option<Solution<P, S, C>> {
        let start = Instant::now();
        while !self.stack.is_empty() {
            if start.elapsed() >= time_limit {
                return None
            }
            match self.state {
                State::FoundSolution => {
                    return self.get_solution()
                },
                State::CoveringColumn => {
                    self.cover_column();
                },
                State::CoveringRow => {
                    self.cover_row();
                },
                State::BacktrackingRow => {
                    self.backtrack_row();
                }
                State::BacktrackingColumn => {
                    self.backtrack_column();
                },
            }
        }
        None
    }

    fn all_solutions(&mut self, time_limit: Duration) -> Vec<Solution<P, S, C>> {
        let start = Instant::now();
        let mut solutions = Vec::new();
        while !self.stack.is_empty() {
            if start.elapsed() >= time_limit {
                break;
            }
            match self.state {
                State::FoundSolution => {
                    self.state = State::BacktrackingRow;
                    if let Some(solution) = self.get_solution() {
                        solutions.push(solution)
                    }
                },
                State::CoveringColumn => {
                    self.cover_column();
                },
                State::CoveringRow => {
                    self.cover_row();
                },
                State::BacktrackingRow => {
                    self.backtrack_row();
                }
                State::BacktrackingColumn => {
                    self.backtrack_column();
                },
            }
        }
        solutions
    }
}

impl<P, S, C> Iterator for DLXCIter<P, S, C> 
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    type Item = (State, Option<Solution<P, S, C>>);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.stack.is_empty() {
            match self.state {
                State::FoundSolution => {
                    self.state = State::BacktrackingRow;
                },
                State::CoveringColumn => {
                    self.cover_column();
                },
                State::CoveringRow => {
                    self.cover_row();
                },
                State::BacktrackingRow => {
                    self.backtrack_row();
                }
                State::BacktrackingColumn => {
                    self.backtrack_column();
                },
            }
            Some((self.state, self.get_solution()))
        }
        else {
            None
        }
    }
}

pub fn dlxc_iter<P, S, C>(sets: Vec<Vec<Item<P, S, C>>>, primary_items: Vec<P>, secondary_items: Vec<S>, colors: Vec<C>) -> DLXCIter<P, S, C>
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    DLXCIter::new(sets, min_length_column, primary_items, secondary_items, colors)
}

pub fn dlxc_iter_randomized<P, S, C>(sets: Vec<Vec<Item<P, S, C>>>, primary_items: Vec<P>, secondary_items: Vec<S>, colors: Vec<C>) -> DLXCIter<P, S, C>
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    DLXCIter::new(sets, min_length_column_randomized, primary_items, secondary_items, colors)
}

pub fn dlxc_first<P, S, C>(sets: Vec<Vec<Item<P, S, C>>>, primary_items: Vec<P>, secondary_items: Vec<S>, 
                           colors: Vec<C>, time_limit: Duration) -> Option<(Vec<Vec<Item<P, S, C>>>, Vec<(S, Option<C>)>)>
where 
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    DLXCIter::new(sets, min_length_column, primary_items, secondary_items, colors)
        .first_solution(time_limit)
}

pub fn dlxc_first_randomized<P, S, C>(sets: Vec<Vec<Item<P, S, C>>>, primary_items: Vec<P>, secondary_items: Vec<S>, 
                                      colors: Vec<C>, time_limit: Duration) -> Option<(Vec<Vec<Item<P, S, C>>>, Vec<(S, Option<C>)>)>
where 
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    DLXCIter::new(sets, min_length_column_randomized, primary_items, secondary_items, colors)
        .first_solution(time_limit)
}

