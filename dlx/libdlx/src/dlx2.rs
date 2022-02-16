use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::dlx_table2::DLXTable;

fn make_index_sets<T>(sets: &Vec<Vec<T>>, items: &Vec<T>) -> Vec<Vec<usize>>
    where T: Hash + Eq + Copy {
    let item_map = items
        .iter()
        .enumerate()
        .map(|(i,item)| (*item,i))
        .collect::<HashMap<T,usize>>();

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

fn get_item_set<T>(index_set: Vec<usize>, items: &Vec<T>) -> Vec<T>
    where T: Copy {
    index_set
        .into_iter()
        .map(|i| items.get(i).cloned().into_iter())
        .flatten()
        .collect()
}

fn get_item_cover<T>(index_cover: Vec<Vec<usize>>, items: &Vec<T>) -> Vec<Vec<T>>
    where T: Copy {
    index_cover
        .into_iter()
        .map(|set| get_item_set(set, items))
        .collect()
}

pub fn dlx<T>(sets: &Vec<Vec<T>>) -> Vec<Vec<Vec<T>>>
    where T: Hash + Eq + Copy {
    let items = get_unique_items(sets);
    let index_sets = make_index_sets(sets, &items);
    let index_covers = dlx_run(index_sets);
    index_covers
        .into_iter()
        .map(|cover| get_item_cover(cover, &items))
        .collect()
}

#[derive(Clone,Copy)]
struct CoverNode {
    item: usize,
    instance: usize
}

fn least_instances_item(table: &DLXTable) -> Option<usize> {
    table.get_current_items()
        .into_iter()
        .map(|item| table.get_instance_count(item))
        .filter(|count| *count > 0)
        .min()
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
        cover.push(table.get_set_items(node_cell.get().instance));
    }

    cover
}

fn try_next_level(table: &mut DLXTable, stack: &mut Vec<Cell<CoverNode>>, node: &mut CoverNode) {
    // Cover new item
    if let Some(next_node) = next_level_node(table) {
        table.cover(next_node.item);
        stack.push(Cell::new(next_node));
    }
    else {
        // No items left to cover.
        // We now uncover the last covered option.
        table.uncover_set(node.instance);
    }
}

pub fn dlx_run(sets: Vec<Vec<usize>>) -> Vec<Vec<Vec<usize>>> {
    let mut table = DLXTable::new(sets, Vec::new());
    let mut covers = Vec::new();
    let item_opt = least_instances_item(&table);
    if item_opt.is_none() {
        return Vec::new();
    }

    let mut stack = Vec::new();
    let item = item_opt.unwrap();
    table.cover(item);
    if let Some(instance) = table.get_next_instance(item) {
        stack.push(Cell::new(CoverNode {
            item,
            instance
        }));
    }
    while let Some(node_cell) = stack.last() {
        let mut node = node_cell.get();
        table.cover_set(node.instance);
        // Select an option to cover
        if let Some(next) = table.get_next_instance(node.instance) {
            node.instance = next;
            node_cell.replace(node);
            try_next_level(&mut table, &mut stack, &mut node);
        }
        else {
            // All options have been tried.
            // Save current cover.
            node_cell.replace(node);
            covers.push(extract_cover(&table, &stack));
            stack.pop();
            // Go back one item.
            table.uncover(node.item);
        }
    }
    covers
}

pub struct DLXIter {
    table: DLXTable,
    stack: Vec<Cell<CoverNode>>
}

pub fn dlx_iter<T>(sets: &Vec<Vec<T>>) -> DLXIter
    where T: Eq + Hash + Copy {
    let index_sets = make_index_sets(sets, &get_unique_items(sets));
    let mut table = DLXTable::new(index_sets);
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