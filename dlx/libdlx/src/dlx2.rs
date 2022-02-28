use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::dlx_table2::DLXTable;

fn make_index_sets<T>(sets: &Vec<Vec<T>>, primary_items: &Vec<T>, secondary_items: &Vec<T>) -> Vec<Vec<usize>>
    where T: Hash + Eq + Copy {
    let mut item_map = primary_items
        .iter()
        .enumerate()
        .map(|(i,item)| (*item,i))
        .collect::<HashMap<T,usize>>();

    let secondaries = secondary_items
        .iter()
        .enumerate()
        .map(|(i,item)| (*item,i))
        .collect::<HashMap<T,usize>>();

    for (item,_) in secondaries.into_iter() {
        item_map.insert(item,item_map.len());
    }

    sets.iter()
        .map(|set| set
            .iter()
            .map(|item| item_map[item])
            .collect::<Vec<usize>>())
        .collect::<Vec<_>>()
}

fn get_item_set<T>(index_set: Vec<usize>, primary_items: &Vec<T>, secondary_items: &Vec<T>) -> Vec<T>
    where T: Copy {
    index_set
        .into_iter()
        .map(|i| primary_items.get(i)
            .or(secondary_items.get(i)).cloned().into_iter())
        .flatten()
        .collect()
}

fn get_item_cover<T>(index_cover: Vec<Vec<usize>>, primary_items: &Vec<T>,
                     secondary_items: &Vec<T>) -> Vec<Vec<T>>
    where T: Copy {
    index_cover
        .into_iter()
        .map(|set| get_item_set(set, primary_items, secondary_items))
        .collect()
}

pub fn dlx<T>(sets: &Vec<Vec<T>>, primary_items: &Vec<T>, secondary_items: &Vec<T>) -> Vec<Vec<Vec<T>>>
    where T: Hash + Eq + Copy {
    let index_sets = make_index_sets(sets, primary_items, secondary_items);
    let index_covers = dlx_run(index_sets, primary_items.len());
    index_covers
        .into_iter()
        .map(|cover|
            get_item_cover(cover, primary_items, secondary_items))
        .collect()
}

#[derive(Clone,Copy,Debug)]
struct CoverNode {
    item: usize,
    instance: usize
}

fn least_instances_item(table: &DLXTable) -> Option<usize> {
    table.get_current_items()
        .into_iter()
        .map(|item| (item, table.get_instance_count(item)))
        .min_by(|(_,c1),(_,c2)| c1.cmp(c2))
        .map(|(item, _)| item)
}

fn next_level_node(table: &DLXTable) -> Option<CoverNode> {
    if let Some(item) = least_instances_item(&table) {
        if let Some(instance) = table.get_next_instance(item) {
            return Some(CoverNode {
                item,
                instance
            })
        }
    }
    None
}

fn extract_cover(table: &DLXTable, stack: &Vec<Cell<CoverNode>>) -> Vec<Vec<usize>> {
    let mut cover = Vec::new();
    for node_cell in stack {
        let node = node_cell.get();
        let set_items = table
            .get_set_items(node.instance)
            .into_iter()
            .map(|item_index| table.get_item_value(item_index).unwrap())
            .collect();
        cover.push(set_items);
    }

    cover
}

#[derive(PartialEq,Eq,Clone,Copy,Debug)]
enum State {
    Covering,
    Backtracking
}

pub fn dlx_run(sets: Vec<Vec<usize>>, primary_items_count: usize) -> Vec<Vec<Vec<usize>>> {
    let mut table = DLXTable::new(sets, primary_items_count);
    let mut covers = Vec::new();
    let mut stack = Vec::new();
    let mut state = State::Covering;
    if let Some(node) = next_level_node(&table) {
        stack.push(Cell::new(node));
        table.cover(node.item);
        table.cover_set(node.instance);
    }
    while let Some(node_cell) = stack.last() {
        let mut node = node_cell.get();
        match state {
            State::Covering => {
                if let Some(next_node) = next_level_node(&table) {
                    // Cover the next node
                    stack.push(Cell::new(next_node));
                    table.cover(next_node.item);
                    table.cover_set(next_node.instance);
                } else {
                    // We reached the end, time to backtrack
                    if let None = least_instances_item(&table) {
                        // Save the solution
                        let cover = extract_cover(&table, &stack);
                        covers.push(cover);
                    }
                    state = State::Backtracking;
                }
            },
            State::Backtracking => {
                if let Some(next_instance) = table.get_next_instance(node.instance) {
                    // Cover the next set
                    table.uncover_set(node.instance);
                    node.instance = next_instance;
                    node_cell.replace(node);
                    table.cover_set(node.instance);
                    state = State::Covering;
                }
                else {
                    // Uncover the last item
                    stack.pop();
                    table.uncover_set(node.instance);
                    table.uncover(node.item);
                }
            }
        }
    }

    covers
}

struct BaseDLXIter {
    table: DLXTable,
    stack: Vec<Cell<CoverNode>>,
    state: State
}

pub struct DLXIter<'a, T> {
    base: BaseDLXIter,
    primary_items: &'a Vec<T>,
    secondary_items: &'a Vec<T>
}

impl<'a, T> DLXIter<'a, T>
    where T: Hash + Eq + Copy {
    pub fn new(sets: &'a Vec<Vec<T>>, primary_items: &'a Vec<T>,
               secondary_items: &'a Vec<T>) -> Self {
        let index_sets = make_index_sets(sets, primary_items, secondary_items);
        let base = BaseDLXIter::new(index_sets, primary_items.len());
        DLXIter { base, primary_items, secondary_items }
    }
}

impl<'a, T> Iterator for DLXIter<'a, T>
    where T: Hash + Eq + Copy {
    type Item = Vec<Vec<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.base.next() {
            Some(cover) =>
                Some(get_item_cover(cover, &self.primary_items, &self.secondary_items)),
            None => None
        }
    }
}

impl BaseDLXIter {
    pub fn new(sets: Vec<Vec<usize>>, primary_items_count: usize) -> Self {
        let table = DLXTable::new(sets, primary_items_count);
        let stack = Vec::new();
        let state = State::Covering;
        let mut this = BaseDLXIter { table, stack, state };
        if let Some(node) = this.next_level_node() {
            this.table.cover(node.item);
            this.stack.push(Cell::new(node));
        }
        this
    }

    fn least_instances_item(&self) -> Option<usize> {
        self.table.get_current_items()
            .into_iter()
            .map(|item| (item, self.table.get_instance_count(item)))
            .min_by(|(_,c1),(_,c2)| c1.cmp(c2))
            .map(|(item,_)| item)
    }

    fn next_level_node(&mut self) -> Option<CoverNode> {
        if let Some(item) = self.least_instances_item() {
            if let Some(instance) = self.table.get_next_instance(item) {
                return Some(CoverNode {
                    item,
                    instance
                })
            }
        }
        None
    }

    fn extract_cover(&self) -> Vec<Vec<usize>> {
        let mut cover = Vec::new();
        for node_cell in &self.stack {
            let node = node_cell.get();
            let set_items = self.table
                .get_set_items(node.instance)
                .into_iter()
                .map(|item_index| self.table.get_item_value(item_index).unwrap())
                .collect();
            cover.push(set_items);
        }
        cover
    }

    fn cover_step(&mut self) -> Option<Vec<Vec<usize>>> {
        let mut cover_opt = None;
        if let Some(node_cell) = self.stack.last() {
            let node = node_cell.get();
            self.table.cover_set(node.instance);
        }
        if let Some(next_node) = self.next_level_node() {
            // Cover the next node
            self.stack.push(Cell::new(next_node));
            self.table.cover(next_node.item);
        } else {
            // We reached the end, time to backtrack
            // Return the solution if the entire table is covered
            if let None = self.least_instances_item() {
                // Save the solution
                cover_opt = Some(self.extract_cover());
            }
            self.state = State::Backtracking;
        }
        cover_opt
    }

    fn backtrack(&mut self) {
        if let Some(node_cell) = self.stack.last() {
            let mut node = node_cell.get();
            if let Some(next_instance) = self.table.get_next_instance(node.instance) {
                // Cover the next set
                self.table.uncover_set(node.instance);
                node.instance = next_instance;
                node_cell.replace(node);
                self.state = State::Covering;
            } else {
                // Uncover the last item
                self.stack.pop();
                self.table.uncover_set(node.instance);
                self.table.uncover(node.item);
            }
        }
    }
}

impl Iterator for BaseDLXIter {
    type Item = Vec<Vec<usize>>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            match self.state {
                State::Covering => {
                    let solution_opt = self.cover_step();
                    if let Some(cover) = solution_opt {
                        return Some(cover)
                    }
                },
                State::Backtracking => {
                    self.backtrack();
                }
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, HashSet};
    use std::hash::Hash;
    use crate::dlx2::{BaseDLXIter, DLXIter};

    fn vec_equality<T: Eq>(first: &Vec<T>, second: &Vec<T>) -> bool {
        for elem in first {
            if !second.contains(elem) {
                return false
            }
        }
        true
    }

    fn deep_vec_equality<T: Eq>(first: &Vec<Vec<T>>, second: &Vec<Vec<T>>) -> bool {
        for vec1 in first {
            let mut equality = false;
            for vec2 in second {
                if vec_equality(vec1, vec2) {
                    equality = true;
                    break;
                }
            }
            if !equality {
                return false
            }
        }
        true
    }

    fn named_dlx_run<T>(sets: &Vec<Vec<T>>, primary_items: &Vec<T>,
                        secondary_items: &Vec<T>) -> Vec<Vec<Vec<T>>>
        where T: Eq + Hash + Copy {
        DLXIter::new(sets, primary_items, secondary_items).collect()
    }

    fn dlx_run(sets: Vec<Vec<usize>>, primary_items_count: usize) -> Vec<Vec<Vec<usize>>> {
        BaseDLXIter::new(sets, primary_items_count).collect()
    }

    fn ordered(solution: Vec<Vec<Vec<usize>>>) -> Vec<HashSet<BTreeSet<usize>>> {
        solution
            .into_iter()
            .map(|cover| cover
                .into_iter()
                .map(|set| set
                    .into_iter()
                    .collect::<BTreeSet<usize>>())
                .collect::<HashSet<_>>())
            .collect::<Vec<_>>()
    }

    #[test]
    fn empty() {
        let result = dlx_run(vec![], 0);
        let expected: Vec<Vec<Vec<usize>>> = vec![];
        assert_eq!(expected, result);
    }

    #[test]
    fn empty_set() {
        let result = dlx_run(vec![vec![]], 0);
        let expected: Vec<Vec<Vec<usize>>> = vec![];
        assert_eq!(expected, result);
    }

    #[test]
    fn one_element() {
        let result = dlx_run(vec![vec![0]], 1);
        let expected = vec![vec![vec![0]]];
        assert_eq!(expected, result);
    }

    #[test]
    fn one_set() {
        let result =
            ordered(dlx_run(vec![vec![0,1,2]], 3));
        let expected = ordered(vec![vec![vec![0,1,2]]]);
        assert_eq!(expected, result);
    }

    #[test]
    fn disjoint_sets() {
        let result =
            ordered(dlx_run(vec![vec![0,1,2], vec![3,4,5]], 6));
        let expected =
            ordered(vec![vec![vec![0,1,2], vec![3,4,5]]]);
        assert_eq!(expected, result);
    }

    #[test]
    fn overlapping_sets() {
        let sets = vec![vec![0,1,2], vec![3,4,5], vec![3,4], vec![5]];
        let result =
            ordered(dlx_run(sets, 6));
        let expected = ordered(vec![
            vec![vec![0,1,2], vec![3,4,5]],
            vec![vec![0,1,2], vec![3,4], vec![5]]]);
        assert_eq!(expected, result);
    }

    #[test]
    fn no_solutions() {
        let sets = vec![vec![0,1,2], vec![3,4,5], vec![4,6]];
        let result = dlx_run(sets, 7);
        let expected: Vec<Vec<Vec<usize>>> = vec![];
        assert_eq!(expected, result);
    }

    #[test]
    fn secondary_items() {
        let sets = vec![vec![0,1,2], vec![3,4,5], vec![3,6], vec![4,7]];
        let result =
            ordered(dlx_run(sets, 5));
        let expected = ordered(vec![
            vec![vec![0,1,2], vec![3,4,5]],
            vec![vec![0,1,2], vec![3,6], vec![4,7]]]);
        assert_eq!(expected, result);
    }

    #[test]
    fn bigger_example() {
        let sets = vec![
            vec![2,4,5], vec![0,3,6], vec![1,2,5],
            vec![0,3], vec![1,6], vec![3,4,6]
        ];
        let result = ordered(dlx_run(sets, 7));
        let expected = ordered(vec![
            vec![vec![0,3], vec![1,6], vec![2,4,5]]]);
        assert_eq!(expected, result);
    }

    #[test]
    fn custom_names() {
        let sets = vec![
            vec!['c', 'e', 'f'], vec!['a', 'd', 'g'], vec!['b', 'c', 'f'],
            vec!['a', 'd'], vec!['b', 'g'], vec!['d', 'e', 'g']
        ];
        let items = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        let result =
            named_dlx_run(&sets, &items, &vec![]);
        let expected = vec![
            vec![vec!['a', 'd'], vec!['b', 'g'], vec!['c', 'e', 'f']]
        ];
        assert_eq!(1, result.len());
        assert!(deep_vec_equality(&expected[0], &result[0]));
    }
}
