use crate::dlx_table::DLXTable;
use std::mem;
use std::collections::HashSet;
use std::iter::Iterator;

// Finds all exact covers of the given sets with the DLX algorithm.
// Solutions are given as a Vec of all possible covers,
// where each covers is a Vec of references to sets that make it up
pub fn exact_cover<'a>(sets: &'a Vec<Vec<usize>>) -> Vec<Vec<&'a Vec<usize>>> {
    let mut table = DLXTable::from(&sets);
    let index_covers = dlx_run(&mut table);
    let mut set_covers = Vec::with_capacity(index_covers.len());

    for cover in index_covers {
        let set_cover = cover.into_iter()
            .map(|index| &sets[index])
            .collect();
        set_covers.push(set_cover);
    }

    set_covers
}

fn previous_level(table: &mut DLXTable, element: usize, set_index: usize) {
    table.uncover_set(element, set_index);
    table.uncover_element(element);
}

fn next_level(table: &mut DLXTable, stack: &mut Vec<(usize,usize)>) {
    let next_element = least_sets_element(table);
    for next_set_index in table.element_sets(next_element) {
        stack.push((next_element,next_set_index));
    }
}

struct DLXIter {
    table: DLXTable,
    stack: Vec<(usize,usize)>,
    current_cover: HashSet<usize>
}

impl DLXIter {
    pub fn from(sets: &Vec<Vec<usize>>) -> Self {
        let table = DLXTable::from(&sets);
        let element = least_sets_element(&table);
        let stack = table.element_sets(element)
            .iter()
            .map(|&set_index| (element,set_index))
            .collect();
        DLXIter { table, stack, current_cover: HashSet::new() }
    }

    fn uncover(&mut self, element: usize, set_index: usize) {
        self.table.uncover_set(element, set_index);
        self.table.uncover_element(element);
    }

    fn cover(&mut self, element: usize, set_index: usize) {
        self.table.cover_element(element);
        self.table.cover_set(element, set_index);
    }
    
    fn next_level(&mut self) {
        let next_element = least_sets_element(&self.table);
        for next_set_index in self.table.element_sets(next_element) {
            self.stack.push((next_element,next_set_index));
        }
    }
}

impl Iterator for DLXIter {
    type Item = HashSet<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut solution: Option<Self::Item> = None;
        while !self.stack.is_empty() || !self.table.elements().is_empty() {
            let (element,set_index) = self.stack.pop().unwrap();
            if self.table.elements().is_empty() {
                self.current_cover.insert(set_index);
                solution = Some(mem::take(&mut self.current_cover));
            }
            else if self.table.has_empty_sets() {
                self.uncover(element, set_index);
            }
            else {
                self.cover(element, set_index);
                self.current_cover.insert(set_index);
                self.next_level();
            }
        }

        solution
    }
}

// iterative DLX implementation
fn dlx_iterative(sets: &Vec<Vec<usize>>) -> Vec<HashSet<usize>> {
    let mut table = DLXTable::from(&sets);
    let mut covers = Vec::<HashSet<usize>>::new();
    
    let element = least_sets_element(&table);
    let mut stack = table.element_sets(element)
        .iter()
        .map(|&set_index| (element,set_index))
        .collect::<Vec<(usize,usize)>>();

    let mut current_cover = HashSet::<usize>::new();
    while !stack.is_empty() {
        let (element,set_index) = stack.pop().unwrap();
        table.cover_element(element);
            table.cover_set(element, set_index);

        if table.elements().is_empty() {
            current_cover.insert(set_index);
            covers.push(mem::take(&mut current_cover));
            previous_level(&mut table, element, set_index);
        }
        else if table.has_empty_sets() {
            previous_level(&mut table, element, set_index);
        }
        else {
            current_cover.insert(set_index);
            next_level(&mut table, &mut stack);
        }
    }

    covers
}

// Finds all exact covers of the given sets with the DLX algorithm.
// Solutions are given as a Vec of all possible covers, /
// with each cover being a sorted Vec of indices of sets that make it up
pub fn dlx(sets: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    dlx_run(&mut DLXTable::from(&sets))
}

// The recursive backtracking algorithm with the Dancing Links technique
fn dlx_run(table: &mut DLXTable) -> Vec<Vec<usize>> {
    if table.elements().is_empty() {
        // return current solution (built in the recursion step)
        vec![vec![]]
    }
    else if table.has_empty_sets() {
        // no solution
        vec![]
    }
    else {
        let mut covers: Vec<Vec<usize>> = Vec::new();
        let elem_index = least_sets_element(table);
        table.cover_element(elem_index);

        let sets = table.element_sets(elem_index);
        for set_index in sets {
            table.cover_set(elem_index, set_index);
            let subcovers = dlx_run(table);
            for mut subcover in subcovers {
                subcover.push(set_index);
                subcover.sort();
                covers.push(subcover);
            }
            table.uncover_set(elem_index, set_index);
        }

        table.uncover_element(elem_index);

        covers.sort();
        covers
    }
}

fn least_sets_element(table: &DLXTable) -> usize {
    table.elements()
        .into_iter()
        .min_by(|&index1, &index2| {
            let count1 = table.element_sets_count(index1);
            let count2 = table.element_sets_count(index2);
            count1.cmp(&count2)
        })
        .unwrap_or(0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_sets() {
        let empty: Vec<Vec<usize>> = vec![];
        let covers = dlx(&empty);

        let expected: Vec<Vec<usize>> = vec![vec![]];
        assert_eq!(expected, covers);
    }

    #[test]
    fn empty_set() {
        let empty: Vec<Vec<usize>> = vec![vec![]];
        let covers = dlx(&empty);

        let expected: Vec<Vec<usize>> = vec![vec![]];
        assert_eq!(expected, covers);
    }

    #[test]
    fn one_element() {
        let sets = vec![vec![0]];
        let covers = dlx(&sets);
        
        let expected = vec![vec![0]];
        assert_eq!(expected, covers);
    }

    #[test]
    fn disjoint_sets() {
        let sets = vec![
            vec![0,1],
            vec![2,3]
        ];
        let covers = dlx(&sets);

        let expected = vec![
            vec![0,1]
        ];
        assert_eq!(expected, covers);
    }

    #[test]
    fn one_solution() {
        let sets = vec![
            vec![0,1],
            vec![2,3],
            vec![0,2]
        ];
        let covers = dlx(&sets);

        let expected = vec![vec![0,1]];
        assert_eq!(expected, covers);
    }

    #[test]
    fn one_solution_2() {
        let sets = vec![
            vec![2,4,5],
            vec![0,3,6],
            vec![1,2,5],
            vec![0,3],
            vec![1,6],
            vec![3,4,6]
        ];
        let covers = dlx(&sets);

        let expected = vec![vec![0,3,4]];
        assert_eq!(expected, covers);
    }
}
