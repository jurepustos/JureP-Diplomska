use std::collections::HashSet;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Element<'a> {
    pub name: &'a str,
    pub len: usize
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct Node {
    header: usize,
    up: usize,
    down: usize,
    left: usize,
    right: usize,
    len: usize
}

impl Node {
    fn root() -> Self {
        Node {
            header: 0,
            up: 0,
            down: 0,
            left: 0,
            right: 0,
            len: 0
        }
    }

    fn header(index: usize, length: usize) -> Self {
        Node {
            header: index,
            up: index,
            down: index,
            left: index-1,
            right: (index+1) % (length+1),
            len: 0
        }
    }

    fn element(header: usize, up: usize, offset: usize, 
        prev: usize, current: usize, elem_count: usize) -> Self {
            Node {
                header: header,
                up,
                down: header,
                left: offset + prev,
                right: offset + ((current+1) % elem_count),
                len: 0
            }
    }
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct DLXTable {
    elements: Vec<String>,
    nodes: Vec<Node>
}


impl DLXTable {
    pub fn new() -> Self {
        DLXTable { 
            elements: vec![], 
            nodes: vec![Node::root()] 
        }
    }

    fn with_capacity(elements: Vec<String>, sets_count: usize) -> Self {
        let mut nodes = Vec::with_capacity(elements.len()+sets_count+1); 
        nodes.push(Node::root());
        
        DLXTable { 
            elements, 
            nodes: nodes
        }
    }

    pub fn from(sets: Vec<Vec<&str>>) -> Self {
        let mut sets = sets;
        for set in sets.iter_mut() {
            set.sort();
        }
        let elements: Vec<String> = set_elements(&sets);
        let mut table = Self::with_capacity(elements, sets.len());
        table.create_headers();
        for set in sets {
            table.add_set(&set);
        }

        table
    }

    pub fn get_elements(&self) -> Vec<Element> {
        let mut elems = Vec::new();
        for (i, elem) in self.elements.iter().enumerate() {
            let header_node = self.nodes[i+1];
            elems.push(Element {
                name: &elem,
                len: header_node.len
            });
        }

        elems
    }

    fn create_headers(&mut self) {
        let length = self.elements.len();
        for i in 0..length {
            self.nodes.push(Node::header(i+1, length));
        }

        let mut root = &mut self.nodes[0];
        root.left = length;
        if length > 0 {
            root.right = 1;
        }
    }

    fn add_set(&mut self, set: &[&str]) {
        if !set.is_empty() {
            let offset = self.nodes.len();
            let elem_count = set.len();
            let last_index = elem_count - 1;
            let mut prev_index = last_index;
            for (current_index, &elem) in set.iter().enumerate() {
                let op_index = self.elements.iter()
                    .position(|element| element == elem);
                
                if let Some(elem_index) = op_index {
                    let header_index = elem_index+1;
                    let mut header_node = &mut self.nodes[header_index];
                    let up_index = header_node.up;
                    header_node.up = offset + current_index;
                    header_node.len += 1;
    
                    let mut up_node = &mut self.nodes[up_index];
                    up_node.down = offset + current_index; 
    
                    self.nodes.push(Node::element(header_index, up_index, 
                        offset, prev_index, current_index, elem_count));
                        
                    prev_index = current_index;
                }
            }
        }
    }


    pub fn cover_element(&mut self, elem_index: usize) {
        let header_index = elem_index+1;
        if header_index <= self.elements.len() {
            self.hide_element(header_index);
            for node_index in self.element_nodes(header_index) {
                self.hide_row(node_index);
            }
        }
    }

    fn hide_element(&mut self, header_index: usize) {
        if header_index <= self.elements.len() {
            let header_node = &self.nodes[header_index];
            let left = header_node.left;
            let right = header_node.right;

            let mut left_node = &mut self.nodes[left];
            left_node.right = right;

            let mut right_node = &mut self.nodes[right];
            right_node.left = left;
        }
    }

    fn element_nodes(&mut self, header_index: usize) -> Vec<usize> {
        let mut indices = Vec::new();
        if header_index <= self.elements.len() {
            let mut next_node = &self.nodes[header_index];
            while next_node.down != header_index {
                indices.push(next_node.down);
                next_node = &self.nodes[next_node.down];
            }
        }

        indices.sort();
        indices
    }

    fn row_nodes(&self, node_index: usize) -> Vec<usize> {
        let mut indices = vec![node_index];
        if let Some(start_node) = self.nodes.get(node_index) {
            let mut next_node = start_node;
            while next_node.right != node_index {
                indices.push(next_node.right);
                next_node = &self.nodes[next_node.right];
            }
        }

        indices.sort();
        indices
    }

    fn hide_row(&mut self, node_index: usize) {
        for index in self.row_nodes(node_index) {
            if index != node_index {
                let node = &self.nodes[index];
                let up = node.up;
                let down = node.down;
                let header = node.header;
    
                let mut up_node = &mut self.nodes[up];
                up_node.down = down;
    
                let mut down_node = &mut self.nodes[down];
                down_node.up = up;
    
                let mut header_node = &mut self.nodes[header];
                header_node.len -= 1;
            }
        }
    }

    pub fn cover_row(&mut self, node_index: usize) {
        for index in self.row_nodes(node_index) {
            if index != node_index {
                let node = &self.nodes[index];
                let elem_index = node.header-1;
                self.cover_element(elem_index);
            }
        }
    }

    pub fn uncover_element(&mut self, elem_index: usize) {
        let header_index = elem_index+1;
        if header_index <= self.elements.len() {
            self.unhide_element(header_index);
            for &node_index in self.element_nodes(header_index).iter().rev() {
                self.unhide_row(node_index);
            }
        }
    }

    fn unhide_element(&mut self, header_index: usize) {
        if header_index <= self.elements.len() {
            let header_node = &self.nodes[header_index];
            let left = header_node.left;
            let right = header_node.right;

            let mut left_node = &mut self.nodes[left];
            left_node.right = header_index;

            let mut right_node = &mut self.nodes[right];
            right_node.left = header_index;
        }
    }

    fn unhide_row(&mut self, node_index: usize) {
        for &index in self.row_nodes(node_index).iter().rev() {
            let node = &self.nodes[index];
            let up = node.up;
            let down = node.down;
            let header = node.header;

            let mut up_node = &mut self.nodes[up];
            up_node.down = node_index;

            let mut down_node = &mut self.nodes[down];
            down_node.up = node_index;

            let mut header_node = &mut self.nodes[header];
            header_node.len += 1;
        }
    }

    pub fn uncover_row(&mut self, node_index: usize) {
        for &index in self.row_nodes(node_index).iter().rev() {
            let node = &self.nodes[index];
            let elem_index = node.header-1;
            self.uncover_element(elem_index);
        }
    }

    fn node_visible(&self, node_index: usize) -> bool {
        if let Some(node) = self.nodes.get(node_index) {
            let left_node = &self.nodes[node.left];
            let right_node = &self.nodes[node.right];
            let up_node = &self.nodes[node.up];
            let down_node = &self.nodes[node.down];

            left_node.right == node_index && right_node.left == node_index
                && up_node.down == node_index && down_node.up == node_index
        }
        else {
            false
        }
    }
}

fn set_elements(sets: &[Vec<&str>]) -> Vec<String> {
    let elements: HashSet<&str> = 
        sets.iter()
            .flat_map(|set| set)
            .map(|&elem| elem)
            .collect();

    let mut elements: Vec<String> = elements.into_iter()
        .map(|elem| elem.to_string())
        .collect();

    elements.sort();
    elements
}


// Tests

mod tests {
    use super::*;

    fn create_table() -> DLXTable {
        let sets = vec![
            vec!["c", "e", "f"],
            vec!["a", "d", "g"],
            vec!["b", "c", "f"],
            vec!["a", "d"],
            vec!["b", "g"],
            vec!["d", "e", "g"]
        ];

        DLXTable::from(sets)
    }

    #[cfg(test)]
    mod creation {
        use super::super::*;

        #[test]
        fn no_sets() {
            let empty: Vec<Vec<&str>> = vec![];
            let table = DLXTable::from(empty);
    
            let expected = DLXTable::new();
            assert_eq!(expected, table);
        }
    
        #[test]
        fn empty_set() {
            let empty: Vec<Vec<&str>> = vec![vec![]];
            let table = DLXTable::from(empty);
    
            let expected = DLXTable::new();
            assert_eq!(expected, table);
        }
    
        #[test]
        fn one_element() {
            let sets: Vec<Vec<&str>> = vec![vec!["a"]];
            let table = DLXTable::from(sets);
    
            let root = Node {
                header: 0,
                up: 0,
                down: 0,
                left: 1,
                right: 1,
                len: 0
            };
            let header = Node {
                header: 1,
                up: 2,
                down: 2,
                left: 0,
                right: 0,
                len: 1
            };
            let node = Node {
                header: 1,
                up: 1,
                down: 1,
                left: 2,
                right: 2,
                len: 0
            };
            let expected = DLXTable {
                elements: vec!["a".to_string()],
                nodes: vec![root, header, node]
            };
    
            assert_eq!(expected, table);
        }

        #[test]
        fn one_set() {
            let sets = vec![vec!["a", "b", "c", "d"]];
            let table = DLXTable::from(sets);

            let elements = vec![
                "a".to_string(), 
                "b".to_string(), 
                "c".to_string(), 
                "d".to_string()
            ];
            let root = Node {
                header: 0,
                up: 0,
                down: 0,
                left: 4,
                right: 1,
                len: 0
            };
            let headers = vec![
                // 1
                Node {
                    header: 1,
                    up: 5,
                    down: 5,
                    left: 0,
                    right: 2,
                    len: 1
                },
                // 2
                Node {
                    header: 2,
                    up: 6,
                    down: 6,
                    left: 1,
                    right: 3,
                    len: 1
                },
                // 3
                Node {
                    header: 3,
                    up: 7,
                    down: 7,
                    left: 2,
                    right: 4,
                    len: 1,
                },
                // 4
                Node {
                    header: 4,
                    up: 8,
                    down: 8,
                    left:3,
                    right: 0,
                    len: 1
                }
            ];
            let element_nodes = vec![
                // 5
                Node {
                    header: 1,
                    up: 1,
                    down: 1,
                    left: 8,
                    right: 6,
                    len: 0
                },
                // 6
                Node {
                    header: 2,
                    up: 2,
                    down: 2,
                    left: 5,
                    right: 7,
                    len: 0
                },
                // 7
                Node {
                    header: 3,
                    up: 3,
                    down: 3,
                    left: 6,
                    right: 8,
                    len: 0
                },
                // 8
                Node {
                    header: 4,
                    up: 4,
                    down: 4,
                    left: 7,
                    right: 5,
                    len: 0
                }
            ];
            let mut nodes = vec![root];
            nodes.extend(headers);
            nodes.extend(element_nodes);

            let expected = DLXTable {
                elements,
                nodes
            };
            assert_eq!(expected, table);
        }

        #[test]
        fn disjoint_sets() {
            let sets = vec![
                vec!["a", "c", "e"],
                vec!["b", "d", "f"]
            ];
            let table = DLXTable::from(sets);

            let elements = vec![
                "a".to_string(), 
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
                "f".to_string()
            ];
            let root = Node {
                header: 0,
                up: 0,
                down: 0,
                left: 6,
                right: 1,
                len: 0
            };
            let headers = vec![
                // 1
                Node {
                    header: 1,
                    up: 7,
                    down: 7,
                    left: 0,
                    right: 2,
                    len: 1
                },
                // 2
                Node {
                    header: 2,
                    up: 10,
                    down: 10,
                    left: 1,
                    right: 3,
                    len: 1
                },
                // 3
                Node {
                    header: 3,
                    up: 8,
                    down: 8,
                    left: 2,
                    right: 4,
                    len: 1
                },
                // 4
                Node {
                    header: 4,
                    up: 11,
                    down: 11,
                    left:3,
                    right: 5,
                    len: 1
                },
                // 5
                Node {
                    header: 5,
                    up: 9,
                    down: 9,
                    left: 4,
                    right: 6,
                    len: 1
                },
                // 6
                Node {
                    header: 6,
                    up: 12,
                    down: 12,
                    left: 5,
                    right: 0,
                    len: 1
                }
            ];
            let element_nodes = vec![
                // 7    
                Node {
                    header: 1,
                    up: 1,
                    down: 1,
                    left: 9,
                    right: 8,
                    len: 0
                },
                // 8
                Node {
                    header: 3,
                    up: 3,
                    down: 3,
                    left: 7,
                    right: 9,
                    len: 0
                },
                // 9
                Node {
                    header: 5,
                    up: 5,
                    down: 5,
                    left: 8,
                    right: 7,
                    len: 0
                },
                // 10
                Node {
                    header: 2,
                    up: 2,
                    down: 2,
                    left: 12,
                    right: 11,
                    len: 0
                },
                // 11
                Node {
                    header: 4,
                    up: 4,
                    down: 4,
                    left: 10,
                    right: 12,
                    len: 0
                },
                // 12
                Node {
                    header: 6,
                    up: 6,
                    down: 6,
                    left: 11,
                    right: 10,
                    len: 0
                }
            ];
            let mut nodes = vec![root];
            nodes.extend(headers);
            nodes.extend(element_nodes);

            let expected = DLXTable {
                elements,
                nodes
            };
            assert_eq!(expected, table);
        }

        #[test]
        fn multiple_solutions() {
            let sets = vec![
                vec!["a", "c", "e"],
                vec!["a", "c", "b"],
                vec!["d", "e"]
            ];
            let table = DLXTable::from(sets);

            let elements = vec![
                "a".to_string(), 
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string()
            ];
            let root = Node {
                header: 0,
                up: 0,
                down: 0,
                left: 5,
                right: 1,
                len: 0
            };
            let headers = vec![
                // 1
                Node {
                    header: 1,
                    up: 9,
                    down: 6, 
                    left: 0,
                    right: 2,
                    len: 2
                },
                // 2
                Node {
                    header: 2,
                    up: 10,
                    down: 10,
                    left: 1,
                    right: 3,
                    len: 1,
                },
                // 3
                Node {
                    header: 3,
                    up: 11,
                    down: 7,
                    left: 2,
                    right: 4,
                    len: 2,
                },
                // 4
                Node {
                    header: 4,
                    up: 12,
                    down: 12,
                    left: 3,
                    right: 5,
                    len: 1
                },
                // 5
                Node {
                    header: 5,
                    up: 13,
                    down: 8,
                    left: 4,
                    right: 0,
                    len: 2
                },
            ];
            let element_nodes = vec![
                // 6
                Node {
                    header: 1,
                    up: 1,
                    down: 9,
                    left: 8,
                    right: 7,
                    len: 0
                },
                // 7
                Node {
                    header: 3,
                    up: 3,
                    down: 11,
                    left: 6,
                    right: 8,
                    len: 0
                },
                // 8
                Node {
                    header: 5,
                    up: 5,
                    down: 13,
                    left: 7,
                    right: 6,
                    len: 0
                },
                // 9
                Node {
                    header: 1,
                    up: 6,
                    down: 1,
                    left: 11,
                    right: 10,
                    len: 0
                },
                // 10
                Node {
                    header: 2,
                    up: 2,
                    down: 2,
                    left: 9,
                    right: 11,
                    len: 0
                },
                // 11
                Node {
                    header: 3,
                    up: 7,
                    down: 3,
                    left: 10,
                    right: 9,
                    len: 0
                },
                // 12
                Node {
                    header: 4,
                    up: 4,
                    down: 4,
                    left: 13,
                    right: 13,
                    len: 0
                },
                // 13
                Node {
                    header: 5,
                    up: 8,
                    down: 5,
                    left: 12,
                    right: 12,
                    len: 0
                }
            ];

            let mut nodes = vec![root];
            nodes.extend(headers);
            nodes.extend(element_nodes);

            let expected = DLXTable {
                elements,
                nodes
            };
            assert_eq!(expected, table);
        }
    }

    #[cfg(test)]
    mod row_nodes {
        use super::*;
        use super::super::*;

        #[test]
        fn one_element() {
            let sets = vec![
                vec!["a"]
            ];
            let table = DLXTable::from(sets);

            assert_eq!(1, table.row_nodes(2).len());
        }

        #[test]
        fn counts_all_elements() {
            let table = create_table();
            let first_row_index = 8;
            let second_row_index = 12;

            let first_row_nodes = table.row_nodes(first_row_index);
            let second_row_nodes = table.row_nodes(second_row_index);
            assert_eq!(3, first_row_nodes.len());
            assert_eq!(3, second_row_nodes.len());
        }

        #[test]
        fn finds_all_elements() {
            let table = create_table();
            let first_row_index = 8;
            let second_row_index = 12;

            let first_row_nodes = table.row_nodes(first_row_index);
            let expected = vec![8,9,10];
            assert_eq!(expected, first_row_nodes);
            
            let second_row_nodes = table.row_nodes(second_row_index);
            let expected = vec![11,12,13];
            assert_eq!(expected, second_row_nodes);
        }
    }

    #[cfg(test)]
    mod cover_element {
        use super::*;
        use super::super::*;

        #[test]
        fn header_unchanged() {
            let mut table = create_table();
            let header_node = table.nodes[1];
            let up_before = header_node.up;
            let down_before = header_node.down; 

            table.cover_element(0);
            let header_node = table.nodes[1];
            
            let up_after = header_node.up;
            assert_eq!(up_before, up_after);
            
            let down_after = header_node.down;
            assert_eq!(down_before, down_after);
        }

        #[test]
        fn header_disconnected() {
            let mut table = create_table();
            table.cover_element(0);

            let header_node = table.nodes[1];
            let left = header_node.left;
            let right = header_node.right;

            let left_node = table.nodes[left];
            let right_node = table.nodes[right];

            assert_eq!(right, left_node.right);
            assert_eq!(left, right_node.left);
        }

        #[test]
        fn element_nodes_unchanged() {
            let mut table = create_table();
            let element_nodes_before = table.element_nodes(1);
            table.cover_element(0);
            let element_nodes_after = table.element_nodes(1);

            assert_eq!(element_nodes_before, element_nodes_after);
        }

        #[test]
        fn rows_disconnected() {
            let mut table = create_table();
            table.cover_element(0);

            for node_index in table.element_nodes(1) {
                for row_node_index in table.row_nodes(node_index) {
                    if row_node_index != node_index {
                        let node = table.nodes[row_node_index];
                        
                        let up_node = table.nodes[node.up];
                        assert_ne!(up_node.down, row_node_index);
                        
                        let down_node = table.nodes[node.down];
                        assert_ne!(down_node.up, row_node_index);
                    }
                }
            }
        }
    }

    #[cfg(test)]
    mod uncover_element {
        use super::*;
        use super::super::*;

        #[test]
        fn header_reconnected() {
            let mut table = create_table();
            table.cover_element(0);
            table.uncover_element(0);

            let header_node = table.nodes[1];
            assert_eq!(0, header_node.left);
            assert_eq!(2, header_node.right);
        }
    }
}



