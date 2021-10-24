type Element = usize;
type Label = usize;

#[derive(Clone, PartialEq, Eq, Debug)]
struct OwnedSet(Label, Vec<bool>);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Set<'a>(Label, &'a [bool]);

impl OwnedSet {
    fn arr(&self) -> &Vec<bool> {
        &self.1
    }

    fn label(&self) -> Label {
        self.0
    }

    fn contains_any(&self, elements: &[Element]) -> bool {
        elements.iter()
            .any(|&elem| self.get_bool(elem))
    }

    fn get_bool(&self, elem: Element) -> bool {
        match self.arr().get(elem) {
            None => false,
            Some(&val) => val
        }
    }
}

impl IntoIterator for OwnedSet {
    type Item = bool;
    type IntoIter = std::vec::IntoIter<bool>;
    fn into_iter<'a>(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        self.1.into_iter()
    }
}

impl From<(Label, Vec<bool>)> for OwnedSet {
    fn from(tuple: (Label, Vec<bool>)) -> Self {
        OwnedSet(tuple.0, tuple.1)
    }
}

impl<'a> Set<'a> {
    fn arr(&self) -> &'a [bool] {
        self.1
    }

    fn label(&self) -> Label {
        self.0
    }

    fn difference(&self, other: Set) -> OwnedSet {
        let arr = self.iter()
            .enumerate()
            .filter(|(i, &_val)| !other.get_bool(*i))
            .map(|(_i, &val)| val)
            .collect();
        
        OwnedSet(self.label(), arr)
    }

    fn iter(&self) -> <Self as IntoIterator>::IntoIter {
        self.arr().into_iter()
    }

    fn contains_any(&self, elements: &[Element]) -> bool {
        elements.iter()
            .any(|&elem| self.get_bool(elem))
    }

    fn get_bool(&self, elem: Element) -> bool {
        match self.arr().get(elem) {
            None => false,
            Some(&val) => val
        }
    }
}

impl<'a> From<(Label, &'a [bool])> for Set<'a> {
    fn from(tuple: (Label, &'a [bool])) -> Self {
        Set(tuple.0, &tuple.1)
    }
}

impl<'a> From<(Label, &'a Vec<bool>)> for Set<'a> {
    fn from(tuple: (Label, &'a Vec<bool>)) -> Self {
        Set(tuple.0, &tuple.1)
    }
}

impl<'a> From<&'a OwnedSet> for Set<'a> {
    fn from(owned_set: &'a OwnedSet) -> Self { 
        Set(owned_set.label(), &owned_set.arr())
    }
}

impl<'a> IntoIterator for Set<'a> {
    type Item = &'a bool;
    type IntoIter = std::slice::Iter<'a, bool>;
    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.arr().into_iter()
    }
}

// Finds all exact covers of the boolean vectors
// returns solutions as vectors of references to the given bool vectors
pub fn exact_cover(vectors: &[Vec<bool>]) -> Vec<Vec<&[bool]>> {
    let sets: Vec<Set> = vectors_to_sets(&vectors);
    find_covers(&sets).iter()
        .map(|solutions| set_slices(&solutions))
        .collect()
}

fn vectors_to_sets<'a>(vectors: &'a [Vec<bool>]) -> Vec<Set<'a>> {
    vectors.iter()
        .enumerate()
        .map(Set::from)
        .collect()
}

fn set_slices<'a>(sets: &[Set<'a>]) -> Vec<&'a [bool]> {
    sets.iter()
        .map(|&set| set.arr())
        .collect()
}



// Finds all exact covers of the given sets 
// and returns references to sets in each cover
fn find_covers<'a>(sets: &[Set<'a>]) -> Vec<Vec<Set<'a>>> {
    if sets.is_empty() {
        Vec::new()
    }
    else if sets.len() == 1 {
        if sets[0].iter().all(|&val| val) {
            vec![vec![sets[0]]]
        }
        else {
            Vec::new()
        }
    }
    else {
        match least_sets_element(&sets) {
            None => vec![Vec::from(sets)],
            Some(selected_elem) => 
                find_associated_covers(&sets, selected_elem)
        }
    }
}

fn find_associated_covers<'a>(sets: &[Set<'a>], elem: Element) -> Vec<Vec<Set<'a>>> {
    let mut all_covers: Vec<Vec<Set>> = Vec::new();
    let candidate_sets = including_sets(&sets, elem);
    for set in candidate_sets {
        let subcovers = branch_covers(sets, set);
        for subcover in subcovers {
            all_covers.push(subcover);
        }
    }

    all_covers
}

fn branch_covers<'a>(sets: &[Set<'a>], branch_set: Set<'a>) -> Vec<Vec<Set<'a>>> {
    let mut covers: Vec<Vec<Set>> = Vec::new();
    let disjoint_sets = cover(&sets, branch_set);
    let remaining_sets: Vec<Set> = 
        disjoint_sets.iter()
            .map(Set::from)
            .collect();
    let subcovers = find_covers(&remaining_sets);
    for subcover in subcovers {
        let set_indices: Vec<Label> = labels(&subcover);
        let mut subcover: Vec<Set> = labeled_sets(&sets, &set_indices);
        subcover.push(branch_set);
        covers.push(subcover);
    }

    covers
}

// Returns the index of the element that is 
// contained in the least sets
fn least_sets_element<'a>(sets: &[Set<'a>]) -> Option<Element> {
    (0..count_elements(&sets))
        .into_iter()
        .map(|elem| count_occurences(&sets, elem))
        .min()
}

// Returns the length of the biggest given slice
fn count_elements(sets: &[Set]) -> usize {
    let max_opt = 
        sets.iter()
            .map(|&set| set.arr().len())
            .max();

    match max_opt {
        Some(max) => max,
        None => 0
    }
}

// Returns the number of slices for which the element at the given index is true
fn count_occurences(sets: &[Set], elem: Element) -> usize {
    sets.iter()
        .filter(|&set| set.get_bool(elem))
        .count()
}


// Returns references to slices that contain the element at the given index
fn including_sets<'a>(sets: &[Set<'a>], elem: Element) -> Vec<Set<'a>> {
    sets.iter()
        .filter(|&set| set.get_bool(elem))
        .map(|&set| set)
        .collect()
}

// Returns a Vector of references to sets that 
// don't contain any of the elements at given indices  
fn cover<'a>(sets: &[Set<'a>], cover_set: Set<'a>) -> Vec<OwnedSet> {
    let cover_elements = set_elements(cover_set);
    let all_elements: Vec<Element> = (0..cover_set.arr().len()).collect();
    sets.iter()
        .filter(|set| 
            !set.contains_any(&cover_elements))
        .map(|set| 
            set.difference(cover_set))
        .filter(|owned_set| 
            owned_set.label() == cover_set.label() || 
            owned_set.contains_any(&all_elements))
        .collect()
}

// Returns indices of elements included in the given set
fn set_elements(set: Set) -> Vec<Element> {
    set.iter()
        .enumerate()
        .filter(|(_i, &val)| val)
        .map(|(i, _val)| i)
        .collect()
}

fn labels(sets: &[Set]) -> Vec<Label> {
    sets.iter()
        .map(|&set| set.label())
        .collect()
}

fn labeled_sets<'a>(sets: &[Set<'a>], labels: &[Label]) -> Vec<Set<'a>> {
    sets.iter()
        .filter(|&set| labels.contains(&set.label()))
        .map(|&set| set)
        .collect()
}

// Tests


#[cfg(test)]
mod tests {
    use super::{exact_cover,including_sets,Set,OwnedSet,cover,set_elements};

    fn covers_equal(cover1: &Vec<&[bool]>, cover2: &Vec<&[bool]>) -> bool {
        cover1.len() == cover2.len() &&
        cover1.iter()
            .all(|set| cover2.contains(&set))
    }

    fn solutions_equal(sol1: &Vec<Vec<&[bool]>>, sol2: &Vec<Vec<&[bool]>>) -> bool {
        sol1.len() == sol2.len() &&
        sol1.iter()
            .all(|cover1| 
                sol2.iter()
                    .any(|cover2| covers_equal(&cover1, &cover2)))
    }

    fn assert_solutions_equal(
        sol1: &Vec<Vec<&[bool]>>, 
        sol2: &Vec<Vec<&[bool]>>) {
            assert!(solutions_equal(&sol1, &sol2));
    }

    #[test]
    fn full_example() {
        let arrays = vec![
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

        let sets = vec![
            Set(0, &arrays[0]),
            Set(1, &arrays[1]),
            Set(2, &arrays[2]),
            Set(3, &arrays[3]),
            Set(4, &arrays[4]),
            Set(5, &arrays[5])
        ];

        let branch_sets: Vec<Set> = including_sets(&sets, 0);
        
        let expected: Vec<Set> = vec![
            sets[0],
            sets[2]
        ];

        assert_eq!(expected, branch_sets);

        let cover1: Vec<OwnedSet> = cover(&sets, sets[0]);
        let sets1: Vec<OwnedSet> = vec![
            OwnedSet(1, vec![true, true, true, true]),
            OwnedSet(3, vec![false, true, true, true]),
            OwnedSet(4, vec![false, false, true, true]),
            OwnedSet(5, vec![true, true, false, false])
        ];

        assert_eq!(sets1, cover1);

        let cover2: Vec<OwnedSet> = cover(&sets, sets[2]);
        let sets2: Vec<OwnedSet> = vec![
            OwnedSet(3, vec![true, true, true]),
            OwnedSet(4, vec![false, true, true])
        ];

        assert_eq!(sets2, cover2);

        let refsets1: Vec<Set> = 
            sets1.iter()
                .map(Set::from)
                .collect();
        let cover11: Vec<OwnedSet> = cover(&refsets1, Set::from(&sets1[0]));
        let sets11: Vec<OwnedSet> = vec![];

        assert_eq!(sets11, cover11);
        
        let elements = set_elements(Set::from(&sets1[3]));
        assert_eq!(vec![0,1], elements);

        let elems = vec![0,1];
        // assert!(!sets1[1].contains_any(&elems));
        
        let rem_sets: Vec<&OwnedSet> = 
            sets1.iter()
                .filter(|set| 
                    !set.contains_any(&elems))
                .collect();
        let exp_sets = vec![
            &sets1[2]
        ];
        assert_eq!(exp_sets, rem_sets);
        

        let cover12: Vec<OwnedSet> = cover(&refsets1, Set::from(&sets1[3]));
        let sets12: Vec<OwnedSet> = vec![
            OwnedSet(4, vec![true, true])
        ];

        assert_eq!(sets12, cover12);
    }

    #[test]
    fn no_sets() {
        let null_set = Vec::new();
        let cover = exact_cover(&null_set);
        let expected = Vec::<Vec<&[bool]>>::new();
        assert_solutions_equal(&expected, &cover);
    }

    #[test]
    fn null_set() {
        let empty: Vec<bool> = Vec::new();
        let null_set = [empty];

        let cover = exact_cover(&null_set);
        let expected: Vec<Vec<&[bool]>> = vec![vec![&null_set[0]]];
        
        assert_solutions_equal(&expected, &cover);
    }

    #[test]
    fn empty_set() {
        let empty_set = [vec![false, false]];

        let cover = exact_cover(&empty_set);
        let expected = Vec::<Vec<&[bool]>>::new();

        assert_solutions_equal(&expected, &cover);
    }

    #[test]
    fn set_with_all_elements() {
        let set = [vec![true, true]];
        let cover = exact_cover(&set);
        let expected: Vec<Vec<&[bool]>> = vec![vec![&set[0]]];
        assert_solutions_equal(&expected, &cover);
    }

    #[test]
    fn set_with_missing_elements() {
        let set = [vec![true, true, false]];
        let cover = exact_cover(&set);
        let expected = Vec::<Vec<&[bool]>>::new();
        assert_solutions_equal(&expected, &cover);
    }

    
    #[test]
    fn disjoint_sets() {
        let sets = vec![
            vec![true, false, true, false],
            vec![false, true, false, true]
        ];
        
        let cover = exact_cover(&sets);
        
        let expected: Vec<Vec<&[bool]>> = vec![
            vec![
                &sets[0], 
                &sets[1]
            ]
        ];
        
        assert!(solutions_equal(&expected, &cover));
    }
    
    #[test]
    fn two_identical_sets() {
        let sets = vec![
            vec![true, false, true, false],
            vec![false, true, false, true],
            vec![false, true, false, true]
        ];
        
        let cover = exact_cover(&sets);
        
        let expected: Vec<Vec<&[bool]>> = vec![
            vec![
                &sets[0],
                &sets[1]
            ], 
            vec![
                &sets[0], 
                &sets[2]
            ]
        ];
        
        assert!(solutions_equal(&expected, &cover));
    }
        
    #[test]

    fn no_solutions() {
        let sets = vec![
            vec![false, true, false, true],
            vec![true, true, true, false]
        ];

        let cover = exact_cover(&sets);
        let expected = Vec::<Vec<&[bool]>>::new();

        assert!(solutions_equal(&expected, &cover));
    }

    #[test]
    fn one_solution() {
        let sets = vec![
            vec![true, false, true, false],
            vec![false, true, false, true],
            vec![true, true, true, false]
        ];

        let cover = exact_cover(&sets);

        let expected: Vec<Vec<&[bool]>> = vec![vec![
            &sets[0],
            &sets[1],
        ]]; 

        assert!(solutions_equal(&expected, &cover));
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

        let cover = exact_cover(&sets);

        // 2 4 5
        // 0 3
        // 1 6
        let expected: Vec<Vec<&[bool]>> = vec![vec![
            &sets[0],
            &sets[3],
            &sets[4]
        ]];

        assert!(solutions_equal(&expected, &cover));
    }

    #[test]
    fn multiple_solutions_simple() {
        let sets = vec![
            vec![true, false, true, false],
            vec![false, true, false, true],
            vec![true, true, false, false],
            vec![false, false, true, true]
        ];

        let cover = exact_cover(&sets);

        let expected: Vec<Vec<&[bool]>> = vec![
            vec![
                &sets[0],
                &sets[1],
            ],
            vec![
                &sets[2],
                &sets[3],
            ]
        ];

        assert!(solutions_equal(&expected, &cover));
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

        let cover = exact_cover(&sets);   

        let expected: Vec<Vec<&[bool]>> = vec![
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

        assert_solutions_equal(&expected, &cover);
    }
}


