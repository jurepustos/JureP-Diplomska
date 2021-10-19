type Set<'a> = &'a [bool];

// Finds all exact covers of the given sets 
// and returns references to sets in each cover
pub fn exact_cover<'a>(sets: &[Set<'a>]) -> Vec<Vec<Set<'a>>> {
    match least_sets_element(&sets) {
        None => Vec::new(),
        Some(selected_elem) => {
            let mut all_covers = Vec::new();
            let candidate_sets = including_sets(&sets, selected_elem);
            for set in candidate_sets {
                let remaining_sets = cover(&sets, &set_elements(set));
                let subcovers = exact_cover(&remaining_sets);
                for subcover in subcovers {
                    let mut cover = subcover;
                    cover.push(set);
                    all_covers.push(cover);
                }
            }
    
            all_covers
        }
    }
}

// Returns the length of the biggest given slice
fn count_all_elements(sets: &[Set]) -> usize {
    if sets.is_empty() {
        0
    }
    else {
        sets.iter()
            .map(|set| set.len())
            .max()
            .unwrap()
    }
}

// Returns the number of true values in the slice
fn count_elements(set: &[bool]) -> usize {
    set.iter()
        .filter(|&&val| val)
        .count()
}

// Returns true if the element at the given index is true. 
// If the element doesn't exist, returns false
fn get_bool(set: &[bool], elem: usize) -> bool {
    match set.get(elem) {
        Some(&val) => val,
        None => false
    }
}

// Returns the number of slices for which the element at the given index is true
fn count_occurences(sets: &[Set], elem: usize) -> usize {
    sets.iter()
        .filter(|set| get_bool(&set, elem))
        .count()
}

// Returns the index of the element that is 
// contained in the least sets
fn least_sets_element<'a>(sets: &[Set<'a>]) -> Option<usize> {
    (0..count_all_elements(&sets))
        .into_iter()
        .map(|elem| count_occurences(&sets, elem))
        .min()
}

// Returns references to slices that contain the element at the given index
fn including_sets<'a>(sets: &[Set<'a>], elem: usize) -> Vec<Set<'a>> {
    sets.iter()
        .filter(|set| get_bool(set, elem))
        .map(|&set_ref| set_ref)
        .collect()
}

// Returns indices of elements included in the given set
fn set_elements(set: &[bool]) -> Vec<usize> {
    set.iter()
        .filter(|&&val| val)
        .enumerate()
        .map(|(i, _val)| i)
        .collect()
}

// Returns a Vector of references to sets that 
// don't contain any of the elements at given indices  
fn cover<'a>(sets: &[Set<'a>], elements: &[usize]) -> Vec<Set<'a>> {
    sets.into_iter()
        .filter(|set|
            elements
                .iter()
                .all(|&elem| !get_bool(set, elem)))
        .map(|&set_ref| set_ref)
        .collect()
}


