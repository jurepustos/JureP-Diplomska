#[derive(Clone,PartialEq,Eq,Debug)]
struct DLXTable<T: Eq + Copy + std::fmt::Debug> {
    names: Vec<Option<T>>,
    left_links: Vec<usize>,
    right_links: Vec<usize>,
    lengths: Vec<usize>,
    up_links: Vec<usize>,
    down_links: Vec<usize>,
    header_links: Vec<usize>,
    primary_count: usize
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

        let node_count = 1 + names.len() + sets.len() + sets
            .iter()
            .map(|set| set
                .iter()
                .map(|_| 1)
                .sum::<usize>())
            .sum::<usize>();

        let mut lengths = vec![0; names.len()];
        let mut left_links = vec![0; names.len()];
        let mut right_links = vec![0; names.len()];
        let mut up_links = vec![0; node_count];
        let mut down_links = vec![0; node_count];
        let mut header_links = vec![0; node_count];

        // header setup
        left_links[0] = names.len() - 1;
        for i in 0..names.len() - 1 {
            left_links[i+1] = i;
            right_links[i] = i+1;
            up_links[i+1] = i+1;
            down_links[i+1] = i+1;
        }

        let mut prev_spacer = names.len();
        
        let mut current_index = names.len() + 1;
        for set in sets {
            if set.len() > 0 {
                for item in set {
                    let header_index = names
                        .iter()
                        .position(|name_opt| name_opt.is_some() && item == name_opt.unwrap())
                        .unwrap();
                    lengths[header_index] += 1;
                    
                    // node setup
                    up_links[current_index] = up_links[header_index];
                    down_links[current_index] = header_index;
                    header_links[current_index] = header_index;

                    // uplink setup
                    down_links[up_links[current_index]] = current_index;
                    
                    // header setup
                    if down_links[header_index] == header_index {
                        down_links[header_index] = current_index;
                    }
                    up_links[header_index] = current_index;
                    
                    current_index += 1;
                }
    
                // spacer
                up_links[current_index] = prev_spacer + 1;
                down_links[prev_spacer] = current_index - 1;
                prev_spacer = current_index;
                current_index += 1;
            }
        }

        DLXTable {
            names,
            left_links,
            right_links,
            lengths,
            up_links,
            down_links,
            header_links,
            primary_count
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
    
    fn hide(&mut self, i: usize) {
        let mut j = i + 1;
        while j != i {
            let header = self.header_links[j];
            // spacer
            if header == 0 {
                j = self.up_links[j];
            }
            else {
                self.up_links[self.down_links[j]] = self.up_links[j];
                self.down_links[self.up_links[j]] = self.down_links[j];
                self.lengths[header] -= 1;

                j += 1;
            }
        }
    }

    fn unhide(&mut self, i: usize) {
        let mut j = i - 1;
        while j != i {
            let header = self.header_links[j];
            // spacer
            if header == 0 {
                j = self.down_links[j];
            }
            else {
                self.lengths[header] += 1;
                self.up_links[self.down_links[j]] = j;
                self.down_links[self.up_links[j]] = j;

                j -= 1;
            }
        }
    }
}

fn choose_column<T: Eq + Copy + std::fmt::Debug>(table: &DLXTable<T>) -> usize {
    let mut j = table.right_links[0];
    let mut s = usize::MAX;
    let mut c = 0;
    while j != 0 {
        if j <= table.primary_count {
            if table.lengths[j] < s {
                c = j;
                s = table.lengths[j];
            }
        }
        j = table.right_links[j];
    }

    c
}

fn get_row<T: Eq + Copy + std::fmt::Debug>(table: &DLXTable<T>, row_node: usize) -> Vec<T> {
    let mut option = vec![table.names[table.header_links[row_node]].unwrap()];
    let mut k = row_node + 1;
    while k != row_node {
        let header = table.header_links[k];
        if header == 0 {
            k = table.up_links[k];
        }
        else {
            option.push(table.names[table.header_links[k]].unwrap());
            k += 1;
        }
    }

    option
}

fn cover_row<T: Eq + Copy + std::fmt::Debug>(table: &mut DLXTable<T>, row_node: usize) {
    let mut j = row_node + 1;
    while j != row_node {
        let header = table.header_links[j];
        if header == 0 {
            j = table.up_links[j];
        }
        else {
            table.cover(header);
            j += 1;
        }
    }
}

fn uncover_row<T: Eq + Copy + std::fmt::Debug>(table: &mut DLXTable<T>, row_node: usize) {
    let mut j = row_node - 1;
    while j != row_node {
        let header = table.header_links[j];
        if header == 0 {
            j = table.down_links[j];
        }
        else {
            table.uncover(header);
            j -= 1;
        }
    }
}



fn search<T: Eq + Copy + std::fmt::Debug>(table: &mut DLXTable<T>, solutions: &mut Vec<Vec<Vec<T>>>, solution: &mut Vec<Vec<T>>) {
    let column = choose_column(table);
    if column == 0 {
        solutions.push(solution.clone());
    }
    else {
        table.cover(column);

        let mut row_node = table.down_links[column];
        while row_node != column {
            cover_row(table, row_node);

            // generate current option
            let option = get_row(table, row_node);

            // recursion
            // go to the next level
            solution.push(option);
            search(table, solutions, solution);
            solution.pop();
            
            uncover_row(table, row_node);

            row_node = table.down_links[row_node];
        }

        table.uncover(column);
    }
}

pub fn dlx<T: Eq + Copy + std::fmt::Debug>(sets: Vec<Vec<T>>, primary_items: Vec<T>, secondary_items: Vec<T>) -> Vec<Vec<Vec<T>>> {
    let mut table = DLXTable::new(sets, primary_items, secondary_items);
    let mut solutions = Vec::new();
    search(&mut table, &mut solutions, &mut Vec::new());
    solutions
}
