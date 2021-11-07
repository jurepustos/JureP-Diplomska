use crate::dlx_table::DLXTable;

pub fn exact_cover<'a>(sets: &'a Vec<Vec<&str>>) -> Vec<Vec<&'a Vec<&'a str>>> {
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

pub fn dlx(sets: &Vec<Vec<&str>>) -> Vec<Vec<usize>> {
    dlx_run(&mut DLXTable::from(&sets))
}

fn dlx_run(table: &mut DLXTable) -> Vec<Vec<usize>> {
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
        let mut covers: Vec<Vec<usize>> = Vec::new();
        let elem_index = mrv(table);
        table.cover_element(elem_index);

        let rows = table.element_nodes(elem_index);
        for node_index in rows {
            table.cover_row(node_index);
            let subcovers = dlx_run(table);
            for mut subcover in subcovers {
                let set_row = table.set_index(node_index).unwrap();
                subcover.push(set_row);
                subcover.sort();
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
        .unwrap_or(0);

    best_header - 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_sets() {
        let empty: Vec<Vec<&str>> = vec![];
        let covers = dlx(&empty);

        let expected: Vec<Vec<usize>> = vec![vec![]];
        assert_eq!(expected, covers);
    }

    #[test]
    fn empty_set() {
        let empty: Vec<Vec<&str>> = vec![vec![]];
        let covers = dlx(&empty);

        let expected: Vec<Vec<usize>> = vec![vec![]];
        assert_eq!(expected, covers);
    }

    #[test]
    fn one_element() {
        let sets = vec![vec!["a"]];
        let covers = dlx(&sets);
        
        // let exp_cover = vec![&sets[0]];
        let expected = vec![vec![0]];
        assert_eq!(expected, covers);
    }
}

