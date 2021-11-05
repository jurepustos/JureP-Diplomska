// mod dlx_table;

use std::cmp::Ordering;
use crate::dlx_table::DLXTable;

pub fn exact_cover(sets: &Vec<Vec<&str>>) -> Vec<Vec<Vec<String>>> {
    let mut table = DLXTable::from(&sets);
    let index_covers = dlx(&mut table);
    let mut str_covers: Vec<Vec<Vec<String>>> = Vec::with_capacity(index_covers.len());
    let elements = table.element_names();
    for cover in index_covers {
        let mut str_cover: Vec<Vec<String>> = Vec::with_capacity(cover.len());
        for index_set in cover {
            let str_set: Vec<String> =
                index_set.into_iter()
                    .map(|index| elements[index].clone())
                    .collect();
            str_cover.push(str_set);
        }
        str_covers.push(str_cover);
    }

    str_covers
}

pub fn dlx(table: &mut DLXTable) -> Vec<Vec<Vec<usize>>> {
    let headers = table.header_nodes();
    if headers.len() == 0 {
        // return current solution (built in the recursion step)
        vec![vec![]]
    }
    else if !table.has_sets() {
        // no solution
        vec![]
    }
    else {
        let mut covers: Vec<Vec<Vec<usize>>> = Vec::new();
        let elem_index = mrv(table);
        table.cover_element(elem_index);

        let rows = table.element_nodes(elem_index);
        for node_index in rows {
            table.cover_row(node_index);
            let subcovers = dlx(table);
            for mut subcover in subcovers {
                let set_row = table.row_elements(node_index);
                subcover.push(set_row);
                subcover.sort_by(cmp_index_sets);
                covers.push(subcover);
            }
            table.uncover_row(node_index);
        }

        table.uncover_element(elem_index);

        covers.sort();
        covers
    }
}

fn mrv(table: &DLXTable) -> usize {
    let best_header = table.header_nodes()
        .into_iter()
        .min_by(|&index1, &index2| {
            let count1 = table.element_nodes_count(index1);
            let count2 = table.element_nodes_count(index2);
            count1.cmp(&count2)
        })
        .unwrap();

    best_header - 1
}

fn cmp_index_sets(set1: &Vec<usize>, set2: &Vec<usize>) -> Ordering {
    let val1 = set1.first();
    let val2 = set2.first();
    match (val1, val2) {
        (None, None) => Ordering::Equal,
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (Some(elem1), Some(elem2)) => elem1.cmp(elem2)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_sets() {
        let empty: Vec<Vec<&str>> = vec![];
        let covers = exact_cover(&empty);

        let expected: Vec<Vec<Vec<String>>> = vec![vec![]];
        assert_eq!(expected, covers);
    }

    #[test]
    fn empty_set() {
        let empty: Vec<Vec<&str>> = vec![vec![]];
        let covers = exact_cover(&empty);

        let expected: Vec<Vec<Vec<String>>> = vec![vec![]];
        assert_eq!(expected, covers);
    }

    #[test]
    fn one_element() {
        let set = vec![vec!["a"]];
        let covers = exact_cover(&set);
        
        let expected = vec![vec![set[0].clone()]];
        assert_eq!(expected, covers);
    }
}

