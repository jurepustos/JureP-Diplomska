use libdlx::*;

#[derive(Clone,Copy,PartialEq,Eq)]
pub struct Cell {
    row: usize,
    column: usize,
    value: usize
}

impl Cell {
    pub fn make(row: usize, column: usize, value: usize) -> Option<Self> {
        if row < 9 && column < 9 && value >= 1 && value <= 9 {
            Some(Cell { row, column, value })
        }
        else {
            None
        }
    }
}

#[derive(Clone,Copy,PartialEq,Eq)]
struct Spec {
    rows: [[bool; 9]; 9],
    columns: [[bool; 9]; 9],
    blocks: [[bool; 9]; 9]
}

impl Spec {
    fn make(cells: &[Cell]) -> Option<Self> {
        let mut rows = [[false; 9]; 9];
        let mut columns = [[false; 9]; 9];
        let mut blocks = [[false; 9]; 9];

        let mut valid = true;
        
        for cell in cells {
            let row = cell.row;
            let column = cell.column;
            let value = cell.value;
            let block = block_index(row, column);

            if !rows[row][value]
            || !columns[column][value]
            || !blocks[block][value] {
                valid = false;
                break;
            }
            else {
                rows[row][value] = true;
                columns[column][value] = true;
                blocks[block][value] = true;
            }
        }

        match valid {
            true => Some(Spec { rows, columns, blocks }),
            false => None
        }
    }

    fn from(cells: &[Cell]) -> Self {
        let mut rows = [[false; 9]; 9];
        let mut columns = [[false; 9]; 9];
        let mut blocks = [[false; 9]; 9];
        
        for cell in cells {
            let row = cell.row;
            let column = cell.column;
            let value = cell.value;
            rows[row][value] = true;
            columns[column][value] = true;
            
            let block = block_index(row, column);
            blocks[block][value] = true;
        }

        Spec { rows, columns, blocks } 
    }

    fn options(&self) -> Vec<Cell> {
        let mut allowed_cells: Vec<Cell> = Vec::new();
        for row in 0..9 {
            for column in 0..9 {
                for value in 1..=9 {
                    if self.is_given(row, column, value)
                    || self.is_allowed(row, column, value) {
                        allowed_cells.push(Cell { row, column, value });
                    }
                }
            }
        }

        allowed_cells
    }

    fn is_given(&self, row: usize, column: usize, value: usize) -> bool {
        self.rows[row][value] && self.columns[column][value]
    }

    fn is_allowed(&self, row: usize, column: usize, value: usize) -> bool {
        let block = block_index(row, column);
        
        !self.rows[row][value] && 
        !self.columns[column][value] && 
        !self.blocks[block][value]
    }
}

// Verifies that the given list of cells represents a valid sudoku puzzle or solution
pub fn verify(cells: &[Cell]) -> bool {
    Spec::make(cells).is_some()
}

// Gives all possible solutions to the given sudoku puzzle
pub fn solve(clues: &[Cell]) -> Vec<Vec<Cell>> {
    if let Some(spec) = Spec::make(clues) {
        let options = spec.options();
        let dlx_sets: Vec<Vec<usize>> = options.iter()
            .map(|cell| dlx_set(cell))
            .collect();

        let dlx_solutions = dlx(&dlx_sets);
        let mut cell_solutions: Vec<Vec<Cell>> = Vec::new();
        for dlx_solution in dlx_solutions {
            let cell_solution: Vec<Cell> = dlx_solution.into_iter()
                .map(|index| options[index])
                .collect();

            cell_solutions.push(cell_solution);
        }

        cell_solutions
    }
    else {
        Vec::new()
    }
}

// Gets the block index of the given coordinates in the sudoku grid
fn block_index(row: usize, column: usize) -> usize {
    3*(row/3) + (column/3)
}

// Transforms the given cell into a set that will be used in DLX for solving 
fn dlx_set(cell: &Cell) -> Vec<usize> {
    let row = cell.row;
    let column = cell.column;
    let value = cell.value;

    let position = row + 9*column;
    let row_item = row + 9*(value-1) + 81;
    let column_item = column + 9*(value-1) + 2*81;
    let block_item = block_index(row, column) + 9*(value-1) + 3*81;
    vec![position, row_item, column_item, block_item]
}



#[cfg(test)]
mod tests {

}