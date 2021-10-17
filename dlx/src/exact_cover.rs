pub struct CoverMatrix<'a> {
    _private: (),
    column_names: Vec<&'a str>,
    rows: Vec<Row>
}

struct Row {
    name: usize,
    array: Vec<bool>
}

impl<'a> CoverMatrix<'a> {
    pub fn from(elements: Vec<&'a str>, sets: Vec<Vec<bool>>) -> CoverMatrix {
        let rows = sets
            .into_iter()
            .enumerate()
            .map(|(i, set)| Row { name: i, array: set })
            .collect::<Vec<Row>>();
        
        CoverMatrix {
            _private: (),
            column_names: elements,
            rows
        }
    }

    pub fn exact_cover(self: &CoverMatrix<'a>) -> Vec<Vec<Vec<&'a str>>> {
        let mut set_solutions = Vec::new();
    
        let row_sets = row_solutions(&self.rows);
        for row_set in row_sets {
            let mut solution: Vec<Vec<&'a str>> = Vec::new();
            for row in row_set {
                let mut column_set: Vec<&'a str> = Vec::new();
                for (i, &element) in self.rows[row].array.iter().enumerate() {
                    if element == true {
                        column_set.push(self.column_names[i]);
                    }
                }
                solution.push(column_set);
            }
            set_solutions.push(solution);
        }

        set_solutions
    }
}


fn row_solutions(matrix: &Vec<Row>) -> Vec<Vec<usize>> {
    if matrix.len() == 0 {
        Vec::new()
    }
    else if matrix.len() == 1 {
        let row = &matrix[0];
        if row.array.iter().all(|val| *val == true) {
            vec![vec![row.name]]
        }
        else {
            Vec::new()
        }
    }
    else{
        let mut solutions = Vec::new();
        let columns = gencolumns(&matrix);

        let best_col = least_trues_index(&columns);
        let select_rows = true_indices(&columns[best_col]);
            
        for &selected_row in select_rows.iter() {
            let row = &matrix[selected_row].array;
            let cols_range: Vec<usize> = (0..row.len()).collect();
            let remove_cols = true_indices(row);
            
            let remaining_cols = diff(&cols_range, &remove_cols);
            let remaining_rows = covered_rows(&matrix, &remove_cols);

            let submatrix = gensubmatrix(&matrix, &remaining_rows, &remaining_cols);

            let selected_row_name = matrix[selected_row].name;
            let subresults = gen_row_subresults(&submatrix, selected_row_name);
            push_to(&mut solutions, subresults);
        }

        solutions
    }
}

fn gen_row_subresults(matrix: &Vec<Row>, row_name: usize) -> Vec<Vec<usize>> {
    let mut subresults = row_solutions(&matrix);
    for result in subresults.iter_mut() {
        result.push(row_name);
    }
    
    subresults
}

fn push_to(dest: &mut Vec<Vec<usize>>, source: Vec<Vec<usize>>) {
    let mut array = source;
    while array.len() > 0 {
        if let Some(res) = array.pop() {
            dest.push(res);
        }
    }
}

fn covered_rows(matrix: &Vec<Row>, covered_cols: &[usize]) -> Vec<usize> {
    let mut remaining_rows: Vec<usize> = (0..matrix.len()).collect();
    for &col in covered_cols {
        let remove_rows = remaining_rows
            .iter()
            .enumerate()
            .filter(|(_i, row)| matrix[**row].array[col] == true)
            .map(|(i, _row)| i)
            .rev()
            .collect::<Vec<usize>>();
        
        for &index in &remove_rows {
            remaining_rows.remove(index);
        }
    }

    remaining_rows
}

fn true_indices(array: &[bool]) -> Vec<usize> {
    array.iter()
        .enumerate()
        .filter(|(_i, col)| **col == true)
        .map(|(i, _col)| i)
        .collect()
}

fn gensubmatrix(matrix: &Vec<Row>, rows: &[usize], cols: &[usize]) -> Vec<Row> {
    let mut submatrix: Vec<Row> = Vec::new();
    for &row in rows {
        let mut new_row: Vec<bool> = Vec::new();
        for &col in cols {
            let matrix_row = &matrix[row].array;
            new_row.push(matrix_row[col]);
        }
        let row_name = matrix[row].name;
        submatrix.push(Row { name: row_name, array: new_row });
    }

    submatrix
}

fn diff(array1: &[usize], array2: &[usize]) -> Vec<usize> {
    let mut result : Vec<usize> = Vec::new();

    let mut j = 0;
    for (i, val) in array1.iter().enumerate() {
        if j < array2.len() {
            if *val == array2[j] {
                j += 1;
            }
            else {
                result.push(i);
            }
        }
        else {
            break;
        }
    }

    result
}

fn gencolumns(matrix: &Vec<Row>) -> Vec<Vec<bool>> {
    if matrix.len() == 0 {
        Vec::new()
    }
    else {
        let row_count = matrix.len();
        let column_count = matrix[0].array.len();
        let mut columns = vec![vec![false; row_count]; column_count];
    
        for (i, row) in matrix.iter().enumerate() {
            for (j, elem) in row.array.iter().enumerate() {
                if *elem == true {
                    columns[j][i] = true;
                }
            }
        }
    
        columns
    }
}

fn least_trues_index(array: &Vec<Vec<bool>>) -> usize {
    array.iter()
        .enumerate()
        .min_by(|(_i, col1), (_j, col2)| count_true(col1).cmp(&count_true(col2)))
        .map(|(i, _col)| i)
        .unwrap()
}

fn count_true(list: &Vec<bool>) -> usize {
    list.into_iter().filter(|val| **val == true).count()
}

