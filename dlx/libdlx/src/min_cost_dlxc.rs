use std::time::Instant;
use std::time::Duration;

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
    costs: Vec<usize>
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

fn add_node<P, S, C>(table: &mut DLXCTable<P, S, C>, index: usize, item: Item<P, S, C>, cost: usize) 
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

    // cost setup
    table.costs[index] = cost;

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
    pub fn new(mut sets: Vec<(Vec<Item<P, S, C>>, usize)>, primary_items: Vec<P>, secondary_items: Vec<S>, colors: Vec<C>) -> Self {
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
            .map(|(set, _)| set
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
            colors: vec![0; node_count],
            costs: vec![0; node_count]
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
        sets.sort_by(|(_, cost1), (_, cost2)| cost1.cmp(cost2));
        for (set, cost) in sets {
            if set.len() > 0 {
                for item in set {
                    add_node(&mut table, current_index, item, cost);
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

    fn commit(&mut self, row_node: usize, threshold: usize) {
        let color = self.colors[row_node];
        let header = self.header_links[row_node];
        if color == 0 {
            self.cover(header, threshold);
        }
        else if color != usize::MAX {
            self.purify(row_node, threshold);
            self.colors[header] = color;
        }
    }

    fn uncommit(&mut self, row_node: usize, threshold: usize) {
        let color = self.colors[row_node];
        let header = self.header_links[row_node];
        if color == 0 {
            self.uncover(header, threshold);
        }
        else if color != usize::MAX {
            self.unpurify(row_node, threshold);
            self.colors[header] = 0;
        }
    }

    fn purify(&mut self, row_node: usize, threshold: usize) {
        let color = self.colors[row_node];
        let header = self.header_links[row_node];
        
        let mut i = self.down_links[header];
        while i != header && self.costs[i] < threshold {
            if self.colors[i] == color {
                self.colors[i] = usize::MAX;
            }
            else {
                self.hide(i);
            }

            i = self.down_links[i];
        }
    }

    fn unpurify(&mut self, row_node: usize, threshold: usize) {
        let color = self.colors[row_node];
        let header = self.header_links[row_node];
        
        let mut i = self.down_links[header];
        while i != header && self.costs[i] < threshold {
            if self.colors[i] == usize::MAX {
                self.colors[i] = color;
            }
            else {
                self.unhide(i);
            }

            i = self.down_links[i];
        }
    }

    fn cover(&mut self, column: usize, threshold: usize) {
        self.left_links[self.right_links[column]] = self.left_links[column];
        self.right_links[self.left_links[column]] = self.right_links[column];

        let mut i = self.down_links[column];
        while i != column && self.costs[i] < threshold {
            self.hide(i);
            i = self.down_links[i];
        }
    }

    fn uncover(&mut self, column: usize, threshold: usize) {
        let mut i = self.down_links[column];
        while i != column && self.costs[i] < threshold {
            self.unhide(i);
            i = self.down_links[i];
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
        let mut i = row_node + 1;
        while i != row_node {
            if self.colors[i] != usize::MAX {
                let header = self.header_links[i];
                if header == 0 {
                    i = self.up_links[i];
                }
                else {
                    self.lengths[header] += 1;
                    self.up_links[self.down_links[i]] = i;
                    self.down_links[self.up_links[i]] = i;
    
                    i += 1;
                }
            }
            else {
                i += 1;
            }
        }
    }

    fn cover_row(&mut self, row_node: usize, threshold: usize) {
        let mut i = row_node + 1;
        while i != row_node {
            let header = self.header_links[i];
            if header == 0 {
                i = self.up_links[i];
            }
            else {
                self.commit(i, threshold);
                i += 1;
            }
        }
    }
    
    fn uncover_row(&mut self, row_node: usize, threshold: usize) {
        let mut i = row_node + 1;
        while i != row_node {
            let header = self.header_links[i];
            if header == 0 {
                i = self.up_links[i];
            }
            else {
                self.uncommit(i, threshold);
                i += 1;
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

fn choose_column<P, S, C>(table: &DLXCTable<P, S, C>, threshold: usize) -> Option<usize> 
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    let mut header = table.right_links[0];
    let mut choice_length = usize::MAX;
    let mut choice = None;
    while header != 0 {
        let mut length = 0;
        let mut i = table.down_links[header];
        while i != header && table.costs[i] < threshold {
            length += 1;
            i = table.down_links[i];
        }

        if length == 0 {
            return None
        }
        else if length < choice_length {
            choice = Some(header);
            choice_length = table.lengths[header];
        }
        else if length == choice_length {
            let choice_cost = table.costs[table.down_links[choice.unwrap()]];
            let header_cost = table.costs[table.down_links[header]];
            if header_cost > choice_cost {
                choice = Some(header);
                choice_length = table.lengths[header];
            }
        } 
        header = table.right_links[header];
    }

    choice
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct Solution<P, S, C>
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    pub rows: Vec<Vec<Item<P, S, C>>>,
    pub colors: Vec<(S, Option<C>)>,
    pub cost: usize
}

// type Solution<P, S, C> = (Vec<Vec<Item<P, S, C>>>, Vec<(S, Option<C>)>);

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
    row_node: usize,
    hiding_threshold: usize,
    covering_threshold: usize
}

pub struct DLXCIter<P, S, C> 
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    table: DLXCTable<P, S, C>,
    stack: Vec<LevelState>,
    state: State,
    current_cost: usize,
    best_cost: usize
}

impl<P, S, C> DLXCIter<P, S, C>
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    pub fn new(sets: Vec<(Vec<Item<P, S, C>>, usize)>, primary_items: Vec<P>, secondary_items: Vec<S>, colors: Vec<C>) -> Self {
        let table = DLXCTable::new(sets, primary_items, secondary_items, colors);
        let stack = Vec::new();
        let state = State::CoveringColumn;
        let current_cost = 0;
        let best_cost = usize::MAX;
        let mut this = DLXCIter { table, stack, state, current_cost, best_cost };
        this.cover_column();
        this
    }

    fn cover_column(&mut self) {
        let hiding_threshold = self.stack
            .last()
            .map(|level| level.hiding_threshold)
            .unwrap_or(usize::MAX);
        if let Some(column) = choose_column(&self.table, hiding_threshold) {
            let row_node = self.table.down_links[column];
            let cost = self.table.costs[row_node];
            if self.best_cost <= self.current_cost + cost {
                self.state = State::BacktrackingRow;
            }
            else {
                let hiding_threshold = 
                    self.best_cost - self.current_cost - cost;
                let covering_threshold = hiding_threshold;
                self.table.cover(column, hiding_threshold);
                self.stack.push(LevelState { 
                    column, 
                    row_node,
                    hiding_threshold,
                    covering_threshold
                });
    
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
        }
        else if self.table.right_links[0] == 0 {
            // all columns are covered
            self.best_cost = self.current_cost;
            self.state = State::FoundSolution;
        }
        else {
            // no solutions exist on this branch
            self.state = State::BacktrackingRow;
        }
    }

    fn cover_row(&mut self) {
        // cover the current row and set up for the next level 
        let level = self.stack.last_mut().unwrap();
        let cost = self.table.costs[level.row_node];
        if self.best_cost <= self.current_cost + cost {
            self.state = State::BacktrackingColumn;
        }
        else {
            let threshold = 
                self.best_cost - self.current_cost - cost;
            level.covering_threshold = threshold;
            self.current_cost += cost;
            self.table.cover_row(level.row_node, threshold);
            self.state = State::CoveringColumn;
        }
    }

    fn backtrack_column(&mut self) {
        // uncover the last covered column
        // and set up to continue
        let level = self.stack.pop().unwrap();
        self.table.uncover(level.column, level.hiding_threshold);
        self.state = State::BacktrackingRow;    
    }

    fn backtrack_row(&mut self) {
        let mut level = self.stack.pop().unwrap();
        self.table.uncover_row(level.row_node, level.covering_threshold);
        self.current_cost -= self.table.costs[level.row_node];
        let row_node = self.table.down_links[level.row_node];
        level.row_node = row_node;
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

    pub fn get_solution(&self) -> Solution<P, S, C> {
        println!("printing solution for cost {}", self.current_cost);
        let solution = self.stack
            .iter()
            .map(|level| level.row_node)
            .map(|i| self.table.get_row(i))
            .collect();
        Solution {
            rows: solution,
            colors: self.table.get_colors(),
            cost: self.current_cost
        }
    }
}

impl<P, S, C> DLXCIter<P, S, C> 
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    pub fn first_solution(mut self, time_limit: Duration) -> Option<Solution<P, S, C>> {
        let start_time = Instant::now();
        while !self.stack.is_empty() {
            if start_time.elapsed() > time_limit {
                return None
            }
            match self.state {
                State::FoundSolution => {
                    return Some(self.get_solution())
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

    pub fn all_solutions(mut self, time_limit: Duration) -> Vec<Solution<P, S, C>> {
        let start_time = Instant::now();
        let mut solutions = Vec::new();
        while !self.stack.is_empty() {
            if start_time.elapsed() > time_limit {
                break;
            }
            match self.state {
                State::FoundSolution => {
                    self.state = State::BacktrackingRow;
                    solutions.push(self.get_solution())
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

    pub fn best_solution(mut self, time_limit: Duration) -> Option<Solution<P, S, C>> {
        let start_time = Instant::now();
        let mut best_solution = None;
        while !self.stack.is_empty() {
            if start_time.elapsed() >= time_limit {
                return None
            }
            match self.state {
                State::FoundSolution => {
                    best_solution = Some(self.get_solution());
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
            // println!("stack: {:?}: {:?}", self.state, self.stack.last());
        }
        best_solution
    }
}

impl<P, S, C> Iterator for DLXCIter<P, S, C> 
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    type Item = Solution<P, S, C>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            match self.state {
                State::FoundSolution => { 
                    self.state = State::BacktrackingRow;
                    return Some(self.get_solution());
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
}

pub fn min_cost_dlxc_iter<P, S, C>(sets: Vec<(Vec<Item<P, S, C>>, usize)>, primary_items: Vec<P>, secondary_items: Vec<S>, colors: Vec<C>) -> DLXCIter<P, S, C>
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    DLXCIter::new(sets, primary_items, secondary_items, colors)
}

pub fn min_cost_dlxc<P, S, C>(sets: Vec<(Vec<Item<P, S, C>>, usize)>, primary_items: Vec<P>, secondary_items: Vec<S>, 
                              colors: Vec<C>, time_limit: Duration) -> Option<Solution<P, S, C>>
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    DLXCIter::new(sets, primary_items, secondary_items, colors).best_solution(time_limit)
}

pub fn min_cost_dlxc_first<P, S, C>(sets: Vec<(Vec<Item<P, S, C>>, usize)>, primary_items: Vec<P>, secondary_items: Vec<S>, 
                                    colors: Vec<C>, time_limit: Duration) -> Option<Solution<P, S, C>>
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    DLXCIter::new(sets, primary_items, secondary_items, colors).first_solution(time_limit)
}
