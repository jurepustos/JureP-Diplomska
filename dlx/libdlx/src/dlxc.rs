use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::mem::take;

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
    colors: Vec<usize>
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
        .unwrap();
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
        if color == 0 {
            let header = self.header_links[row_node];
            self.cover(header);
        }
        else if color != usize::MAX {
            self.purify(row_node);
        }
    }

    fn uncommit(&mut self, row_node: usize) {
        let color = self.colors[row_node];
        if color == 0 {
            let header = self.header_links[row_node];
            self.uncover(header);
        }
        else if color != usize::MAX {
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

    fn get_color(&self, row_node: usize) -> Option<C> {
        let color_index = self.colors[row_node];
        if color_index != usize::MAX {
            self.color_names[color_index]
        }
        else {
            None
        }
    }

    fn get_item(&self, row_node: usize) -> Item<P, S, C> {
        match self.names[self.header_links[row_node]] {
            Some(Item::Primary(item)) => Item::Primary(item),
            Some(Item::Secondary(item)) => {
                match self.get_color(row_node) {
                    Some(color) => Item::ColoredSecondary(item, color), 
                    None => Item::Secondary(item)
                }
            },
            _ => panic!("None or ColoredSecondary access in headers. Something went horribly wrong.")
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
    state: State
}

impl<P, S, C> DLXCIter<P, S, C>
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    pub fn new(sets: Vec<Vec<Item<P, S, C>>>, primary_items: Vec<P>, secondary_items: Vec<S>, colors: Vec<C>) -> Self {
        let table = DLXCTable::new(sets, primary_items, secondary_items, colors);
        DLXCIter::from_table(table)
    }

    pub fn from_table(mut table: DLXCTable<P, S, C>) -> Self {
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
        DLXCIter { table, stack, state }
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

    pub fn get_solution(&self) -> Option<Vec<Vec<Item<P, S, C>>>> {
        if let State::FoundSolution = self.state {
            Some(self.stack
                .iter()
                .map(|level| level.row_node)
                .map(|i| self.table.get_row(i))
                .collect())
        }
        else {
            None
        }
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

impl<P, S, C> Iterator for DLXCIter<P, S, C> 
where
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    type Item = (State, Option<Vec<Vec<Item<P, S, C>>>>);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.stack.is_empty() {
            match self.state {
                State::FoundSolution => {
                    self.state = State::BacktrackingRow;
                },
                State::CoveringColumn => {
                    if let Some(column) = choose_column(&self.table) {
                        // cover next column
                        self.cover_column(column);
                    }
                    else {
                        // all columns are covered
                        self.state = State::FoundSolution;
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
    DLXCIter::new(sets, primary_items, secondary_items, colors)
}

pub fn dlxc_first<P, S, C>(sets: Vec<Vec<Item<P, S, C>>>, primary_items: Vec<P>, 
                          secondary_items: Vec<S>, colors: Vec<C>) -> Option<Vec<Vec<Item<P, S, C>>>>
where 
P: Eq + Copy + std::fmt::Debug,
S: Eq + Copy + std::fmt::Debug,
C: Eq + Copy + std::fmt::Debug {
    let mut table = DLXCTable::new(sets, primary_items, secondary_items, colors);
    search(&mut table, &mut Vec::new())
        .map(|solution| solution
            .into_iter()
            .map(|node_index| table.get_row(node_index))
            .collect())
}

pub use mp::*;

mod mp {
    use std::mem::replace;
    use std::sync::mpsc::channel;
    use std::sync::mpsc::Sender;
    use std::mem::take;
    use crate::dlxc::LevelState;
    use crate::dlxc::DLXCIter;
    use std::sync::mpsc::Receiver;
    use crate::dlxc::Item;
    use crate::dlxc::State;
    use std::thread::spawn;
    use std::sync::mpsc::SyncSender;
    use std::thread::JoinHandle;
    use std::sync::mpsc::sync_channel;
    use crate::dlxc::choose_column;
    use std::collections::VecDeque;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;
    use std::sync::Arc;
    use crate::dlxc::DLXCTable;

    type Solution<P, S, C> = Vec<Vec<Item<P, S, C>>>;

    struct Task {
        columns: Vec<usize>,
        rows: Vec<usize>
    }
    
    fn get_tasks<P, S, C>(table: &DLXCTable<P, S, C>) -> VecDeque<Task>
    where
    P: Eq + Copy + std::fmt::Debug,
    S: Eq + Copy + std::fmt::Debug,
    C: Eq + Copy + std::fmt::Debug {
        let mut queue = VecDeque::<Task>::new();
        if let Some(column) = choose_column(table) {
            let mut row_node = table.down_links[column];
            while row_node != column {
                queue.push_front(Task {
                    columns: vec![column],
                    rows: vec![row_node]
                });

                row_node = table.down_links[row_node];
            }
        }

        queue
    }

    fn mp_task<P, S, C>(table: DLXCTable<P, S, C>, starting_stack: Vec<LevelState>, 
                        tx: &SyncSender<Solution<P, S, C>>, run_lock: &AtomicBool)
    where
    P: Eq + Copy + std::fmt::Debug,
    S: Eq + Copy + std::fmt::Debug,
    C: Eq + Copy + std::fmt::Debug {
        let mut iter = DLXCIter::from_table(table);
        iter.stack = starting_stack;
        while run_lock.load(Ordering::Relaxed) == true {
            match iter.next() {
                Some((State::FoundSolution, Some(solution))) => {
                    let _res = tx.send(solution);
                },
                None => break,
                _ => ()
            }
        }
    }

    fn spawn_thread<P, S, C>(table: &DLXCTable<P, S, C>, task: &Task, tx: &SyncSender<Solution<P, S, C>>, 
                             join_tx: &Sender<usize>, thread_index: usize, run_lock: &Arc<AtomicBool>) -> JoinHandle<()>
    where
    P: 'static + Eq + Copy + std::fmt::Debug + Send,
    S: 'static + Eq + Copy + std::fmt::Debug + Send,
    C: 'static + Eq + Copy + std::fmt::Debug + Send {
        let mut thread_table = table.clone();
        let mut starting_stack = Vec::new();
        for i in 0..task.columns.len() {
            starting_stack.push(LevelState {
                column: task.columns[i],
                row_node: task.rows[i]
            });
            
            thread_table.cover(task.columns[i]);
            thread_table.cover_row(task.rows[i]);
        }
        let thread_tx = tx.clone();
        let thread_join_tx = join_tx.clone();
        let thread_run_lock = Arc::clone(run_lock);
        spawn(move || {
            mp_task(thread_table, starting_stack, &thread_tx, &thread_run_lock);
            let _res = thread_join_tx.send(thread_index);
        })
    }

    fn get_solution<P, S, C>(table: &DLXCTable<P, S, C>, threads: &mut Vec<Option<JoinHandle<()>>>, queue: &mut VecDeque<Task>, 
                             tx: &SyncSender<Solution<P, S, C>>, rx: &Receiver<Solution<P, S, C>>,
                             join_tx: &Sender<usize>, join_rx: &Receiver<usize>,
                             run_lock: &Arc<AtomicBool>) -> Option<Solution<P, S, C>>
    where
    P: 'static + Eq + Copy + std::fmt::Debug + Send,
    S: 'static + Eq + Copy + std::fmt::Debug + Send,
    C: 'static + Eq + Copy + std::fmt::Debug + Send {
        let mut threads_finished = 0;
        while threads_finished < threads.len() {
            if let Ok(solution) = rx.try_recv() {
                run_lock.store(false, Ordering::Relaxed);
                return Some(solution)
            }

            if let Ok(i) = join_rx.try_recv() {
                if let Some(task) = queue.pop_back() {
                    let thread = spawn_thread(&table, &task, &tx, &join_tx, threads.len(), &run_lock);
                    threads.push(Some(thread));
                }

                let thread = replace(&mut threads[i], None);
                let _res = thread.unwrap().join();
                threads_finished += 1;
            }
        }
        None
    }

    pub fn dlxc_first_mp<P, S, C>(sets: Vec<Vec<Item<P, S, C>>>, primary_items: Vec<P>, 
                                  secondary_items: Vec<S>, colors: Vec<C>, 
                                  thread_count: usize) -> Option<Vec<Vec<Item<P, S, C>>>> 
    where
    P: 'static + Eq + Copy + std::fmt::Debug + Send,
    S: 'static + Eq + Copy + std::fmt::Debug + Send,
    C: 'static + Eq + Copy + std::fmt::Debug + Send {   
        let table = DLXCTable::new(sets, primary_items, secondary_items, colors);
        let run_lock = Arc::new(AtomicBool::new(true));
        let (tx, rx) = sync_channel(1000);
        let (join_tx, join_rx) = channel();
    
        let mut threads = Vec::new();
        let mut queue = get_tasks(&table);
    
        while threads.len() < thread_count {
            if let Some(task) = queue.pop_back() {
                let thread = spawn_thread(&table, &task, &tx, &join_tx, threads.len(), &run_lock);
                threads.push(Some(thread));
            }
            else {
                break;
            }   
        }

        let result = get_solution(&table, &mut threads, &mut queue, &tx, &rx, &join_tx, &join_rx, &run_lock);
        
        let running_threads = threads
            .into_iter()
            .filter(Option::is_some)
            .map(Option::unwrap);
        for thread in running_threads {
            let _res = thread.join();
        }

        result
    }
    
    pub struct DLXCIterMP<P, S, C>
    where
    P: 'static + Eq + Copy + std::fmt::Debug + Send,
    S: 'static + Eq + Copy + std::fmt::Debug + Send,
    C: 'static + Eq + Copy + std::fmt::Debug + Send {
        table: DLXCTable<P, S, C>,
        run_lock: Arc<AtomicBool>,
        queue: VecDeque<Task>,
        tx: SyncSender<Solution<P, S, C>>,
        rx: Receiver<Solution<P, S, C>>,
        join_tx: Sender<usize>,
        join_rx: Receiver<usize>,
        threads: Vec<Option<JoinHandle<()>>>,
        threads_finished: usize
    }

    impl<P, S, C> DLXCIterMP<P, S, C>
    where
    P: 'static + Eq + Copy + std::fmt::Debug + Send,
    S: 'static + Eq + Copy + std::fmt::Debug + Send,
    C: 'static + Eq + Copy + std::fmt::Debug + Send {
        pub fn stop(self) {
            drop(self)
        }

        fn spawn_thread(&self, task: Task, thread_index: usize) -> JoinHandle<()> {
            let mut thread_table = self.table.clone();
            let mut starting_stack = Vec::new();
            for i in 0..task.columns.len() {
                starting_stack.push(LevelState {
                    column: task.columns[i],
                    row_node: task.rows[i]
                });

                thread_table.cover(task.columns[i]);
                thread_table.cover_row(task.rows[i]);
            }
            
            let thread_tx = self.tx.clone();
            let thread_join_tx = self.join_tx.clone();
            let thread_run_lock = Arc::clone(&self.run_lock);
            spawn(move || {
                mp_task(thread_table, starting_stack, &thread_tx, &thread_run_lock);
                let _res = thread_join_tx.send(thread_index);
            })
        }
    }

    impl<P, S, C> Iterator for DLXCIterMP<P, S, C>
    where
    P: 'static + Eq + Copy + std::fmt::Debug + Send,
    S: 'static + Eq + Copy + std::fmt::Debug + Send,
    C: 'static + Eq + Copy + std::fmt::Debug + Send {
        type Item = Solution<P, S, C>;
        fn next(&mut self) -> Option<Self::Item> {
            while self.threads_finished < self.threads.len() {
                if let Ok(solution) = self.rx.try_recv() {
                    return Some(solution)
                }
                
                if let Ok(i) = self.join_rx.try_recv() {
                    if let Some(task) = self.queue.pop_back() {
                        let thread = self.spawn_thread(task, self.threads.len());
                        self.threads.push(Some(thread));
                    }

                    let thread = replace(&mut self.threads[i], None);
                    let _res = thread.unwrap().join();
                    self.threads_finished += 1;
                }
            }
            None
        }
    }

    impl<P, S, C> Drop for DLXCIterMP<P, S, C>
    where
    P: 'static + Eq + Copy + std::fmt::Debug + Send,
    S: 'static + Eq + Copy + std::fmt::Debug + Send,
    C: 'static + Eq + Copy + std::fmt::Debug + Send {
        fn drop(&mut self) {
            self.run_lock.store(false, Ordering::Relaxed);
            let threads = take(&mut self.threads);
            let running_threads = threads
                .into_iter()
                .filter(Option::is_some)
                .map(Option::unwrap);
            for thread in running_threads {
                let _res = thread.join();
            }
        }
    }
    
    pub fn dlxc_iter_mp<P, S, C>(sets: Vec<Vec<Item<P, S, C>>>, primary_items: Vec<P>, 
                                secondary_items: Vec<S>, colors: Vec<C>, 
                                thread_count: usize) -> DLXCIterMP<P, S, C>
    where
    P: 'static + Eq + Copy + std::fmt::Debug + Send,
    S: 'static + Eq + Copy + std::fmt::Debug + Send,
    C: 'static + Eq + Copy + std::fmt::Debug + Send {   
        let table = DLXCTable::new(sets, primary_items, secondary_items, colors);
        let run_lock = Arc::new(AtomicBool::new(true));
        let (tx, rx) = sync_channel(1000);
        let (join_tx, join_rx) = channel();
        
        let mut threads = Vec::new();
        let mut queue = get_tasks(&table);

        while threads.len() < thread_count {
            if let Some(task) = queue.pop_back() {
                let thread = spawn_thread(&table, &task, &tx, &join_tx, threads.len(), &run_lock);
                threads.push(Some(thread));
            }
            else {
                break;
            }
        }

        DLXCIterMP {
            table, 
            run_lock, 
            tx,
            rx, 
            join_tx,
            join_rx,
            queue,
            threads,
            threads_finished: 0
        }
    }
}
