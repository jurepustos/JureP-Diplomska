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

    let mut i = item_map.len();
    for (item,_) in secondaries.into_iter() {
        item_map.insert(item,i);
    }

    sets.iter()
        .map(|set| set
            .iter()
            .map(|item| item_map[item])
            .collect::<Vec<usize>>())
        .collect::<Vec<_>>()
}

fn get_unique_items<T>(sets: &Vec<Vec<T>>) -> Vec<T>
    where T: Eq + Hash + Copy {
    sets.iter()
        .flatten()
        .collect::<HashSet<&T>>()
        .into_iter()
        .map(|t| *t)
        .collect::<Vec<T>>()
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

pub struct DLXIter {
    table: DLXTable,
    stack: Vec<Cell<CoverNode>>
}

pub fn dlx_iter<T>(sets: &Vec<Vec<T>>, primary_items: &Vec<T>, secondary_items: &Vec<T>) -> DLXIter
    where T: Eq + Hash + Copy {
    let index_sets = make_index_sets(sets, primary_items, secondary_items);
    let mut table = DLXTable::new(index_sets, primary_items.len());
    let mut stack = Vec::new();
    if let Some(item) = least_instances_item(&table) {
        table.cover(item);
        if let Some(instance) = table.get_next_instance(item) {
            stack.push(Cell::new(CoverNode {
                item,
                instance
            }));
        }
    }
    DLXIter { table, stack }
}

impl DLXIter {
    fn least_instances_item(&self) -> Option<usize> {
        self.table.get_current_items()
            .into_iter()
            .map(|item| self.table.get_instance_count(item))
            .filter(|count| *count > 0)
            .min()
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

    fn next_level(&mut self, node: CoverNode) {
        if let Some(next_node) = self.next_level_node() {
            self.table.cover(next_node.item);
            self.stack.push(Cell::new(next_node));
        }
        else {
            // No items left to cover.
            // We now uncover the last covered option.
            self.table.uncover_set(node.instance);
        }
    }

    fn extract_cover(&self) -> Vec<Vec<usize>> {
        let mut cover = Vec::new();
        for node_cell in &self.stack {
            cover.push(self.table.get_set_items(node_cell.get().instance));
        }
        cover
    }
}

impl Iterator for DLXIter {
    type Item = Vec<Vec<usize>>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node_cell) = self.stack.last() {
            let mut node = node_cell.get();
            self.table.cover_set(node.instance);
            // Select an option to cover
            if let Some(next) = self.table.get_next_instance(node.instance) {
                node.instance = next;
                node_cell.replace(node);
                self.next_level(node);
            }
            else {
                // All options have been tried.
                // Save current cover.
                let cover = self.extract_cover();
                self.stack.pop();
                // Go back one item.
                self.table.uncover(node.item);
                return Some(cover);
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use crate::dlx2::dlx_run;

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
        let result = dlx_run(vec![vec![0,1,2]], 3);
        let expected = vec![vec![vec![0,1,2]]];
        assert_eq!(expected, result);
    }

    #[test]
    fn disjoint_sets() {
        let result = dlx_run(vec![vec![0,1,2], vec![3,4,5]], 6);
        let expected = vec![vec![vec![0,1,2], vec![3,4,5]]];
        assert_eq!(expected, result);
    }

    #[test]
    fn overlapping_sets() {
        let result =
            dlx_run(vec![vec![0,1,2], vec![3,4,5], vec![3,4], vec![5]], 6);
        let expected =
            vec![vec![vec![0,1,2], vec![3,4,5]], vec![vec![0,1,2], vec![3,4], vec![5]]];
        assert_eq!(expected, result);
    }

    #[test]
    fn no_solutions() {
        let result = dlx_run(vec![vec![0,1,2], vec![3,4,5], vec![4,6]], 7);
        let expected: Vec<Vec<Vec<usize>>> = vec![];
        assert_eq!(expected, result);
    }

    #[test]
    fn secondary_items() {
        let sets = vec![vec![0,1,2], vec![3,4,5], vec![3,6], vec![4,7]];
        let result =
            dlx_run(sets, 5);
        let expected =
            vec![vec![vec![0,1,2], vec![3,4,5]], vec![vec![0,1,2], vec![3,6], vec![4,7]]];
        assert_eq!(expected, result);
    }
}
