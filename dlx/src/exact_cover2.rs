type Element = usize;
type Label = usize;
type Set<'a> = (Label, &'a [bool]);

// Finds all exact covers of the boolean vectors
// returns solutions as vectors of references to the given bool vectors
pub fn exact_cover(vectors: &[Vec<bool>]) -> Vec<Vec<&[bool]>> {
    let sets: Vec<Set> = vectors_to_sets(&vectors);
    find_covers(&sets)
        .iter()
        .map(|solution_sets| set_slices(&solution_sets))
        .collect()
}

fn vectors_to_sets<'a>(vectors: &'a [Vec<bool>]) -> Vec<Set<'a>> {
    vectors.iter()
        .map(|set| &set[..])
        .enumerate()
        .collect()
}

fn set_slices<'a>(sets: &[Set<'a>]) -> Vec<&'a [bool]> {
    sets.iter()
        .map(|&(_i, set)| set)
        .collect()
}


// Finds all exact covers of the given sets 
// and returns references to sets in each cover
fn find_covers<'a>(sets: &[Set<'a>]) -> Vec<Vec<Set<'a>>> {
    if sets.is_empty() {
        Vec::new()
    }
    else if sets.len() == 1 {
        if sets[0].1.iter().all(|&val| val) {
            vec![vec![sets[0]]]
        }
        else {
            Vec::new()
        }
    }
    else {
        match least_sets_element(&sets) {
            None => Vec::new(),
            Some(selected_elem) => find_associated_covers(sets, selected_elem)
        }
    }
}

fn find_associated_covers<'a>(sets: &[Set<'a>], elem: Element) -> Vec<Vec<Set<'a>>> {
    let mut all_covers: Vec<Vec<Set>> = Vec::new();
    let candidate_sets = including_sets(&sets, elem);
    for set in candidate_sets {
        let remaining_sets = cover(&sets, set);
        let subcovers = find_covers(&set_refs(&remaining_sets));
        for subcover in subcovers {
            let set_indices: Vec<Label> = labels(&subcover);
            let mut full_cover: Vec<Set> = labeled_sets(&sets, &set_indices);
            full_cover.push(set);
            all_covers.push(full_cover);
        }
    }

    all_covers
}

// Returns the length of the biggest given slice
fn count_all_elements(sets: &[Set]) -> usize {
    if sets.is_empty() {
        0
    }
    else {
        sets.iter()
            .map(|(_i, set)| set.len())
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
fn get_bool(set: &[bool], elem: Element) -> bool {
    match set.get(elem) {
        Some(&val) => val,
        None => false
    }
}

// Returns the number of slices for which the element at the given index is true
fn count_occurences(sets: &[Set], elem: Element) -> usize {
    sets.iter()
        .filter(|(_i, set)| get_bool(&set, elem))
        .count()
}

// Returns the index of the element that is 
// contained in the least sets
fn least_sets_element<'a>(sets: &[Set<'a>]) -> Option<Element> {
    (0..count_all_elements(&sets))
        .into_iter()
        .map(|elem| count_occurences(&sets, elem))
        .min()
}

// Returns references to slices that contain the element at the given index
fn including_sets<'a>(sets: &[Set<'a>], elem: Element) -> Vec<Set<'a>> {
    sets.iter()
        .filter(|(_i, set)| get_bool(&set, elem))
        .map(|&set_ref| set_ref)
        .collect()
}

fn set_refs(sets: &[(Label, Vec<bool>)]) -> Vec<Set> {
    sets.iter()
        .map(|(i, set)| (*i, &set[..]))
        .collect()
}

// Returns indices of elements included in the given set
pub fn set_elements(set: Set) -> Vec<Element> {
    let (_i, slice) = set;
    slice.iter()
        .enumerate()
        .filter(|(_i, &val)| val)
        .map(|(i, _val)| i)
        .collect()
}

// Returns a Vector of references to sets that 
// don't contain any of the elements at given indices  
pub fn cover<'a>(sets: &[Set<'a>], cover_set: Set<'a>) -> Vec<(Label, Vec<bool>)> {
    let cover_elements = set_elements(cover_set);
    sets.into_iter()
        .filter(|&(_i, set)| !contains_any(&set, &cover_elements))
        .map(|&(i, set)| (i, set_reduce(&set)))
        .collect()
}

fn contains_any(set: &[bool], elements: &[Element]) -> bool {
    elements.iter()
        .any(|&elem| get_bool(&set, elem))
}

fn set_reduce(set: &[bool]) -> Vec<bool> {
    set.iter()
        .filter(|&&val| val)
        .map(|&val| val)
        .collect()
}

fn labels(sets: &[Set]) -> Vec<Label> {
    sets.into_iter()
        .map(|&(i, _set)| i)
        .collect()
}

fn labeled_sets<'a>(sets: &[Set<'a>], labels: &[Label]) -> Vec<Set<'a>> {
    sets.iter()
        .filter(|(i, _set)| labels.contains(i))
        .map(|&set| set)
        .collect()
}

// Tests


#[cfg(test)]
mod tests {

    #[test]
    fn no_sets() {
        let null_set = Vec::new();
        let cover = super::exact_cover(&null_set);
        assert_eq!(Vec::<Vec<&[bool]>>::new(), cover);
    }

    #[test]
    fn null_set() {
        let null_set = [Vec::new()];

        let cover = super::exact_cover(&null_set);
        assert_eq!(vec![vec![Vec::<bool>::new()]], cover);
    }

    #[test]
    fn empty_set() {
        let empty_set = [vec![false, false]];

        let cover = super::exact_cover(&empty_set);
        assert_eq!(Vec::<Vec<&[bool]>>::new(), cover);
    }

    #[test]
    fn set_with_all_elements() {
        let set = [vec![true, true]];
        let cover = super::exact_cover(&set);
        let expected: Vec<Vec<&[bool]>> = vec![vec![&set[0]]];
        assert_eq!(expected, cover);
    }

    #[test]
    fn set_with_missing_elements() {
        let set = [vec![true, true, false]];
        let cover = super::exact_cover(&set);
        assert_eq!(Vec::<Vec<&[bool]>>::new(), cover);
    }

    #[test]
    fn disjoint_sets() {
        let sets = vec![
            vec![true, false, true, false],
            vec![false, true, false, true]
        ];

        let cover = super::exact_cover(&sets);

        let expected = vec![vec![&sets[0], &sets[1]]];

        assert_eq!(expected, cover);
    }

    #[test]

    fn no_solutions() {
        let sets = vec![
            vec![false, true, false, true],
            vec![true, true, true, false]
        ];

        let cover = super::exact_cover(&sets);

        assert_eq!(Vec::<Vec<&[bool]>>::new(), cover);
    }

    #[test]
    fn one_solution() {
        let sets = vec![
            vec![true, false, true, false],
            vec![false, true, false, true],
            vec![true, true, true, false]
        ];

        let cover = super::exact_cover(&sets);

        let expected =vec![vec![
            &sets[0],
            &sets[1],
        ]]; 

        assert_eq!(expected, cover);
    }

    #[test]
    fn one_solution_bigger() {
        let sets = vec![
            // 0: 2 4 5
            vec![false, false, true, false, true, true, false],
            // 1: 0 3 6 
            vec![true, false, false, true, false, false, true],
            // 2: 1 2 5
            vec![false, true, true, false, false, true, false],
            // 3: 0 3
            vec![true, false, false, true, false, false, false],
            // 4: 1 6
            vec![false, true, false, false, false, false, true],
            // 5: 3 4 6
            vec![false, false, false, true, true, false, true]
        ];

        let cover = super::exact_cover(&sets);

        // 2 4 5
        // 0 3
        // 1 6
        let expected = vec![vec![
            &sets[0],
            &sets[3],
            &sets[4]
        ]];

        assert_eq!(expected, cover);
    }

    #[test]
    fn multiple_solutions_simple() {
        let sets = vec![
            vec![true, false, true, false],
            vec![false, true, false, true],
            vec![true, true, false, false],
            vec![false, false, true, true]
        ];

        let cover = super::exact_cover(&sets);

        let expected = vec![
            vec![
                &sets[0],
                &sets[1]
            ],
            vec![
                &sets[2],
                &sets[3]
            ]
        ];

        assert_eq!(expected, cover);
    }

    #[test]
    fn multiple_solutions() {
        let sets = vec![
            // 0: 0
            vec![true, false, false, false, false],
            // 1: 1 2 3 4
            vec![false, true, true, true, true],
            // 2: 0 1
            vec![true, true, false, false, false],
            // 3: 2 3 4
            vec![false, false, true, true, true],
            // 4: 3 4
            vec![false, false, false, true, true],
            // 5: 1 2
            vec![false, true, true, false, false]
        ];

        let cover = super::exact_cover(&sets);   

        let expected = vec![
            // 0
            // 1 2 3 4
            vec![
                &sets[0],
                &sets[1]
            ],
            // 0 1
            // 2 3 4
            vec![
                &sets[2],
                &sets[3]
            ],
            // 0
            // 3 4
            // 1 2
            vec![
                &sets[0],
                &sets[4],
                &sets[5],
            ]
        ];

        assert_eq!(expected, cover);
    }
}


