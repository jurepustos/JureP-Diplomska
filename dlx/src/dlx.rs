mod dlx_table;

use dlx_table::DLXTable;

pub fn dlx(table: &mut DLXTable) -> Vec<Vec<&str>> {
    let elements = table.get_elements();
    if elements.len() == 0 {
        // current solution and return
    }
    else {
        // recursion
    }
}

