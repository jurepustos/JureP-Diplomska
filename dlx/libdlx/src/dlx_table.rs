#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct Node {
    header: usize,
    up: usize,
    down: usize,
    left: usize,
    right: usize,
    len: Option<usize>,
    set: Option<usize>
}

impl Node {
    fn root() -> Self {
        Node {
            header: 0,
            up: 0,
            down: 0,
            left: 0,
            right: 0,
            len: None,
            set: None
        }
    }

    fn header(index: usize, length: usize) -> Self {
        Node {
            header: index,
            up: index,
            down: index,
            left: index-1,
            right: (index+1) % (length+1),
            len: Some(0),
            set: None
        }
    }
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct DLXTable {
    element_count: usize,
    nodes: Vec<Node>,
    set_heads: Vec<usize>
}


impl DLXTable {
    pub fn new() -> Self {
        DLXTable { 
            element_count: 0, 
            nodes: vec![Node::root()],
            set_heads: Vec::new()
        }
    }

    fn with_capacity(elements: usize, sets_count: usize) -> Self {
        let mut nodes = Vec::with_capacity(elements+sets_count+1); 
        nodes.push(Node::root());
        
        DLXTable { 
            element_count: elements,
            nodes,
            set_heads: Vec::new()
        }
    }

    pub fn from(sets: &Vec<Vec<usize>>) -> Self {
        let elements = element_count(&sets);
        let mut table = Self::with_capacity(elements, sets.len());
        table.create_headers();
        for set in sets {
            table.add_set(&set);
        }

        table
    }

    fn get_header(&self, element: usize) -> Option<&Node> {
        if element < self.element_count {
            self.nodes.get(element+1)
        }
        else {
            None
        }
    }

    fn get_header_mut(&mut self, element: usize) -> Option<&mut Node> {
        if element < self.element_count {
            self.nodes.get_mut(element+1)
        }
        else {
            None
        }
    }

    fn get_element(&self, node: &Node) -> usize {
        node.header - 1
    }

    pub fn elements(&self) -> Vec<usize> {
        let mut indices = Vec::new();
        let root = self.nodes[0];
        let mut node = &self.nodes[root.right];
        while node.right != root.right {
            let elem = self.get_element(node);
            indices.push(elem);

            let index = node.right;
            node = &self.nodes[index];
        }

        indices
    }

    pub fn element_sets(&self, element: usize) -> Vec<usize> {
        let mut indices = Vec::new();
        if let Some(header) = self.get_header(element) {
            let mut node = header;
            while node.down != header.header {
                node = &self.nodes[node.down];
                if let Some(set_index) = node.set {
                    indices.push(set_index);
                }
            }
        }
        
        indices.sort();
        indices
    }

    pub fn has_empty_sets(&self) -> bool {
        let elements = self.elements();
        if elements.is_empty() {
            false
        }
        else {
            elements.into_iter()
                .any(|elem| self.get_header(elem).unwrap().len == Some(0))
        }
    }
    
    fn create_headers(&mut self) {
        let length = self.element_count;
        for i in 0..length {
            self.nodes.push(Node::header(i+1, length));
        }

        let mut root = &mut self.nodes[0];
        root.left = length;
        if length > 0 {
            root.right = 1;
        }
    }

    fn add_set(&mut self, set: &[usize]) {
        if !set.is_empty() {
            self.set_heads.push(self.nodes.len());
            let set_index = self.set_heads.len() - 1;

            let offset = self.nodes.len();
            let set_len = set.len();
            for (current_index, &element) in set.iter().enumerate() {
                self.insert_node(current_index, element, set_index, offset, set_len);
            }
        }
    }

    fn insert_node(&mut self, index: usize, element: usize, set_index: usize, offset: usize, set_len: usize) {
        let mut header = self.get_header_mut(element).unwrap();
        let header_index = header.header;
        let up = header.up;
        header.up = offset + index;
        
        let len = header.len.unwrap_or(0);
        header.len = Some(len+1);

        let mut up_node = &mut self.nodes[up];
        up_node.down = offset + index;

        let prev = 
            if index == 0 {
                set_len-1
            }
            else {
                index-1
            };
        let next = (index+1) % set_len;
        let left = offset + prev;
        let right = offset + next;
        
        let new_node = Node {
            header: header_index,
            up,
            down: header_index,
            left,
            right,
            len: None,
            set: Some(set_index)
        };

        self.nodes.push(new_node);
    }


    pub fn cover_element(&mut self, element: usize) {
        if element < self.element_count {
            self.hide_element(element);
            for set_index in self.element_sets(element) {
                self.hide_row(element, set_index);
            }
        }
    }

    fn hide_element(&mut self, element: usize) {
        if let Some(header) = self.get_header(element) {
            let left = header.left;
            let right = header.right;

            let mut left_node = &mut self.nodes[left];
            left_node.right = right;

            let mut right_node = &mut self.nodes[right];
            right_node.left = left;
        }
    }

    pub fn element_sets_count(&self, element: usize) -> usize {
        match self.get_header(element) {
            Some(header) => header.len.unwrap_or(0),
            None => 0 
        }
    }

    fn element_nodes(&self, element: usize) -> Vec<usize> {
        let mut indices = Vec::new();
        if element < self.element_count {
            let header = self.get_header(element).unwrap();
            let mut next_node = header;
            while next_node.down != header.header {
                indices.push(next_node.down);
                let down = next_node.down;
                next_node = &self.nodes[down];
            }
        }

        indices.sort();
        indices
    }

    pub fn row_nodes(&self, set_index: usize) -> Vec<usize> {
        let mut indices = vec![];
        if let Some(&set_head) = self.set_heads.get(set_index) {
            indices.push(set_head);
            if let Some(start_node) = self.nodes.get(set_head) {
                let mut next_node = start_node;
                while next_node.right != set_head {
                    let right = next_node.right;
                    indices.push(right);
                    next_node = &self.nodes[right];
                }
            }
        }

        indices.sort();
        indices
    }

    pub fn set_elements(&self, set_index: usize) -> Vec<usize> {
        let mut elem_indices = Vec::new();
        let row_nodes = self.row_nodes(set_index);
        for node_index in row_nodes {
            let node = &self.nodes[node_index];
            let element = self.get_element(node);
            elem_indices.push(element);
        } 

        elem_indices.sort();
        elem_indices
    }

    fn hide_row(&mut self, element: usize, set_index: usize) {
        for index in self.row_nodes(set_index) {
            let node = &self.nodes[index];
            if self.get_element(node) != element {
                let up = node.up;
                let down = node.down;   
                let element = self.get_element(node);
    
                let mut up_node = &mut self.nodes[up];
                up_node.down = down;
    
                let mut down_node = &mut self.nodes[down];
                down_node.up = up;
    
                let mut header = self.get_header_mut(element).unwrap();
                let len = header.len.unwrap_or(1);
                header.len = Some(len-1);
            }
        }
    }

    pub fn cover_row(&mut self, element: usize, set_index: usize) {
        for index in self.row_nodes(set_index) {
            let node = &self.nodes[index];
            let node_element = self.get_element(node);
            if node_element != element {
                self.cover_element(node_element);
            }
        }
    }

    pub fn uncover_element(&mut self, element: usize) {
        if element < self.element_count {
            self.unhide_element(element);
            let elem_sets = self.element_sets(element);
            for set_index in elem_sets {
                self.unhide_row(element, set_index);
            }
        }
    }

    fn unhide_element(&mut self, element: usize) {
        if let Some(header) = &self.get_header(element) {
            let left = header.left;
            let right = header.right;
            let header_index = header.header;

            let mut left_header = &mut self.nodes[left];
            left_header.right = header_index;

            let mut right_header = &mut self.nodes[right];
            right_header.left = header_index;
        }
    }

    fn unhide_row(&mut self, element: usize, set_index: usize) {
        for index in self.row_nodes(set_index) {
            let node = &self.nodes[index];
            if self.get_element(node) != element {
                let up = node.up;
                let down = node.down;
                let header = node.header;
    
                let mut up_node = &mut self.nodes[up];
                up_node.down = index;
    
                let mut down_node = &mut self.nodes[down];
                down_node.up = index;
    
                let mut header_node = &mut self.nodes[header];
                let len = header_node.len.unwrap_or(0);
                header_node.len = Some(len+1);
            }
        }
    }

    pub fn uncover_row(&mut self, element: usize, set_index: usize) {
        for index in self.row_nodes(set_index).into_iter().rev() {
            let node = &self.nodes[index];
            if self.get_element(node) != element {
                let node = &self.nodes[index];
                let node_element = self.get_element(node);
                self.uncover_element(node_element);
            }
        }
    }
}

fn element_count(sets: &[Vec<usize>]) -> usize {
    match max_element(sets) {
        None => 0,
        Some(max) => max+1
    }
}

fn max_element(sets: &[Vec<usize>]) -> Option<usize> {
    sets.into_iter()
        .flat_map(|set|
            set.into_iter())
        .map(|&elem| elem)
        .max()
}

// Tests

mod tests {
    use super::*;

    fn create_table() -> DLXTable {
        let sets = vec![
            vec![2, 4, 5],
            vec![0, 3, 6],
            vec![1, 2, 5],
            vec![0, 3],
            vec![1, 6],
            vec![3, 4, 6]
        ];

        DLXTable::from(&sets)
    }

    #[cfg(test)]
    mod creation {
        use super::super::*;

        #[test]
        fn no_sets() {
            let empty: Vec<Vec<usize>> = vec![];
            let table = DLXTable::from(&empty);
    
            let expected = DLXTable::new();
            assert_eq!(expected, table);
        }
    
        #[test]
        fn empty_set() {
            let empty: Vec<Vec<usize>> = vec![vec![]];
            let table = DLXTable::from(&empty);
    
            let expected = DLXTable::new();
            assert_eq!(expected, table);
        }
    
        #[test]
        fn one_element() {
            let sets = vec![vec![0]];
            let table = DLXTable::from(&sets);
    
            let root = Node {
                header: 0,
                up: 0,
                down: 0,
                left: 1,
                right: 1,
                len: None,
                set: None
            };
            let header = Node {
                header: 1,
                up: 2,
                down: 2,
                left: 0,
                right: 0,
                len: Some(1),
                set: None
            };
            let node = Node {
                header: 1,
                up: 1,
                down: 1,
                left: 2,
                right: 2,
                len: None,
                set: Some(0)
            };
            let expected = DLXTable {
                element_count: 1,
                nodes: vec![root, header, node],
                set_heads: vec![2]
            };
    
            assert_eq!(expected, table);
        }

        #[test]
        fn one_set() {
            let sets = vec![vec![0,1,2,3]];
            let table = DLXTable::from(&sets);

            let root = Node {
                header: 0,
                up: 0,
                down: 0,
                left: 4,
                right: 1,
                len: None,
                set: None
            };
            let headers = vec![
                // 1
                Node {
                    header: 1,
                    up: 5,
                    down: 5,
                    left: 0,
                    right: 2,
                    len: Some(1),
                    set: None
                },
                // 2
                Node {
                    header: 2,
                    up: 6,
                    down: 6,
                    left: 1,
                    right: 3,
                    len: Some(1),
                    set: None
                },
                // 3
                Node {
                    header: 3,
                    up: 7,
                    down: 7,
                    left: 2,
                    right: 4,
                    len: Some(1),
                    set: None
                },
                // 4
                Node {
                    header: 4,
                    up: 8,
                    down: 8,
                    left:3,
                    right: 0,
                    len: Some(1),
                    set: None
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
                    len: None,
                    set: Some(0)
                },
                // 6
                Node {
                    header: 2,
                    up: 2,
                    down: 2,
                    left: 5,
                    right: 7,
                    len: None, 
                    set: Some(0)
                },
                // 7
                Node {
                    header: 3,
                    up: 3,
                    down: 3,
                    left: 6,
                    right: 8,
                    len: None,
                    set: Some(0)
                },
                // 8
                Node {
                    header: 4,
                    up: 4,
                    down: 4,
                    left: 7,
                    right: 5,
                    len: None,
                    set: Some(0)
                }
            ];
            let mut nodes = vec![root];
            nodes.extend(headers);
            nodes.extend(element_nodes);

            let expected = DLXTable {
                element_count: 4,
                nodes,
                set_heads: vec![5]
            };
            assert_eq!(expected, table);
        }

        #[test]
        fn disjoint_sets() {
            let sets = vec![
                vec![0,2,4],
                vec![1,3,5]
            ];
            let table = DLXTable::from(&sets);

            let root = Node {
                header: 0,
                up: 0,
                down: 0,
                left: 6,
                right: 1,
                len: None,
                set: None
            };
            let headers = vec![
                // 1
                Node {
                    header: 1,
                    up: 7,
                    down: 7,
                    left: 0,
                    right: 2,
                    len: Some(1),
                    set: None
                },
                // 2
                Node {
                    header: 2,
                    up: 10,
                    down: 10,
                    left: 1,
                    right: 3,
                    len: Some(1),
                    set: None
                },
                // 3
                Node {
                    header: 3,
                    up: 8,
                    down: 8,
                    left: 2,
                    right: 4,
                    len: Some(1),
                    set: None
                },
                // 4
                Node {
                    header: 4,
                    up: 11,
                    down: 11,
                    left:3,
                    right: 5,
                    len: Some(1),
                    set: None
                },
                // 5
                Node {
                    header: 5,
                    up: 9,
                    down: 9,
                    left: 4,
                    right: 6,
                    len: Some(1),
                    set: None
                },
                // 6
                Node {
                    header: 6,
                    up: 12,
                    down: 12,
                    left: 5,
                    right: 0,
                    len: Some(1),
                    set: None
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
                    len: None,
                    set: Some(0)
                },
                // 8
                Node {
                    header: 3,
                    up: 3,
                    down: 3,
                    left: 7,
                    right: 9,
                    len: None,
                    set: Some(0)
                },
                // 9
                Node {
                    header: 5,
                    up: 5,
                    down: 5,
                    left: 8,
                    right: 7,
                    len: None,
                    set: Some(0)
                },
                // 10
                Node {
                    header: 2,
                    up: 2,
                    down: 2,
                    left: 12,
                    right: 11,
                    len: None,
                    set: Some(1)
                },
                // 11
                Node {
                    header: 4,
                    up: 4,
                    down: 4,
                    left: 10,
                    right: 12,
                    len: None,
                    set: Some(1)
                },
                // 12
                Node {
                    header: 6,
                    up: 6,
                    down: 6,
                    left: 11,
                    right: 10,
                    len: None,
                    set: Some(1)
                }
            ];
            let mut nodes = vec![root];
            nodes.extend(headers);
            nodes.extend(element_nodes);

            let expected = DLXTable {
                element_count: 6,
                nodes,
                set_heads: vec![7,10]
            };
            assert_eq!(expected, table);
        }

        #[test]
        fn multiple_solutions() {
            let sets = vec![
                vec![0,2,4],
                vec![0,1,2],
                vec![3,4]
            ];
            let table = DLXTable::from(&sets);

            let root = Node {
                header: 0,
                up: 0,
                down: 0,
                left: 5,
                right: 1,
                len: None,
                set: None
            };
            let headers = vec![
                // 1
                Node {
                    header: 1,
                    up: 9,
                    down: 6, 
                    left: 0,
                    right: 2,
                    len: Some(2),
                    set: None
                },
                // 2
                Node {
                    header: 2,
                    up: 10,
                    down: 10,
                    left: 1,
                    right: 3,
                    len: Some(1),
                    set: None
                },
                // 3
                Node {
                    header: 3,
                    up: 11,
                    down: 7,
                    left: 2,
                    right: 4,
                    len: Some(2),
                    set: None
                },
                // 4
                Node {
                    header: 4,
                    up: 12,
                    down: 12,
                    left: 3,
                    right: 5,
                    len: Some(1),
                    set: None
                },
                // 5
                Node {
                    header: 5,
                    up: 13,
                    down: 8,
                    left: 4,
                    right: 0,
                    len: Some(2),
                    set: None
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
                    len: None,
                    set: Some(0)
                },
                // 7
                Node {
                    header: 3,
                    up: 3,
                    down: 11,
                    left: 6,
                    right: 8,
                    len: None,
                    set: Some(0)
                },
                // 8
                Node {
                    header: 5,
                    up: 5,
                    down: 13,
                    left: 7,
                    right: 6,
                    len: None,
                    set: Some(0)
                },
                // 9
                Node {
                    header: 1,
                    up: 6,
                    down: 1,
                    left: 11,
                    right: 10,
                    len: None,
                    set: Some(1)
                },
                // 10
                Node {
                    header: 2,
                    up: 2,
                    down: 2,
                    left: 9,
                    right: 11,
                    len: None,
                    set: Some(1)
                },
                // 11
                Node {
                    header: 3,
                    up: 7,
                    down: 3,
                    left: 10,
                    right: 9,
                    len: None,
                    set: Some(1)
                },
                // 12
                Node {
                    header: 4,
                    up: 4,
                    down: 4,
                    left: 13,
                    right: 13,
                    len: None,
                    set: Some(2)
                },
                // 13
                Node {
                    header: 5,
                    up: 8,
                    down: 5,
                    left: 12,
                    right: 12,
                    len: None,
                    set: Some(2)
                }
            ];

            let mut nodes = vec![root];
            nodes.extend(headers);
            nodes.extend(element_nodes);

            let expected = DLXTable {
                element_count: 5,
                nodes,
                set_heads: vec![6,9,12]
            };
            assert_eq!(expected, table);
        }
    }

    #[cfg(test)]
    mod element_sets {
        use super::*;

        #[test]
        fn one_element() {
            let sets = vec![vec![0]];
            let table = DLXTable::from(&sets);

            let element_sets = table.element_sets(0);
            
            let expected = vec![0];
            assert_eq!(expected, element_sets); 
        }

        #[test]
        fn finds_all_containing_sets() {
            let table = create_table();

            let element_sets = table.element_sets(0);
            let expected = vec![1,3];
            assert_eq!(expected, element_sets);

            let element_sets = table.element_sets(3);
            let expected = vec![1,3,5];
            assert_eq!(expected, element_sets);
        }
    }

    #[cfg(test)]
    mod row_nodes {
        use super::*;

        #[test]
        fn one_element() {
            let sets = vec![vec![0]];
            let table = DLXTable::from(&sets);

            assert_eq!(1, table.row_nodes(0).len());
        }

        #[test]
        fn counts_all_elements() {
            let table = create_table();

            let first_row_nodes = table.row_nodes(0);
            let second_row_nodes = table.row_nodes(1);
            assert_eq!(3, first_row_nodes.len());
            assert_eq!(3, second_row_nodes.len());
        }

        #[test]
        fn finds_all_elements() {
            let table = create_table();

            let first_row_nodes = table.row_nodes(0);
            let expected = vec![8,9,10];
            assert_eq!(expected, first_row_nodes);
            
            let second_row_nodes = table.row_nodes(1);
            let expected = vec![11,12,13];
            assert_eq!(expected, second_row_nodes);
        }
    }

    #[cfg(test)]
    mod cover_element {
        use super::*;

        #[test]
        fn header_unchanged() {
            let mut table = create_table();
            table.cover_element(0);

            let header_node = table.nodes[1];
            
            assert_eq!(11, header_node.down);
            assert_eq!(17, header_node.up);
            assert_eq!(0, header_node.left);
            assert_eq!(2, header_node.right);
        }

        #[test]
        fn header_horizontal_disconnected() {
            let mut table = create_table();
            table.cover_element(0);

            let left_node = table.nodes[0];
            let right_node = table.nodes[2];

            assert_eq!(2, left_node.right);
            assert_eq!(0, right_node.left);
        }

        #[test]
        fn header_vertical_connected() {
            let mut table = create_table();
            table.cover_element(0);
            
            let down_node = table.nodes[11];
            let up_node = table.nodes[17];

            assert_eq!(1, down_node.up);
            assert_eq!(1, up_node.down);
        }

        #[test]
        fn element_nodes_unchanged() {
            let mut table = create_table();
            table.cover_element(0);
            let element_nodes = table.element_nodes(0);

            let expected = vec![11,17];
            assert_eq!(expected, element_nodes);
        }


        #[test]
        fn header_children_connected() {
            let mut table = create_table();
            table.cover_element(0);

            let first_node = table.nodes[11];
            let second_node = table.nodes[17];

            assert_eq!(1, first_node.up);
            assert_eq!(17, first_node.down);
            assert_eq!(11, second_node.up);
            assert_eq!(1, second_node.down);
        }

        #[test]
        fn row_nodes_vertical_disconnected() {
            let mut table = create_table();
            table.cover_element(0);

            let second_header = table.nodes[4];
            assert_eq!(21, second_header.down);
            assert_eq!(21, second_header.up);

            let third_header = table.nodes[7];
            assert_eq!(20, third_header.down);
        }
    }

    #[cfg(test)]
    mod uncover_element {
        use super::*;

        #[test]
        fn recovers_original_state() {
            let orig_table = create_table();
            let mut table = create_table();
            table.cover_element(0);
            table.uncover_element(0);

            assert_eq!(orig_table, table);
        }

        #[test]
        fn header_reconnected() {
            let mut table = create_table();
            table.cover_element(0);
            table.uncover_element(0);

            let header_node = table.nodes[1];
            assert_eq!(0, header_node.left);
            assert_eq!(2, header_node.right);
        }

        #[test]
        fn rows_reconnected() {
            let mut table = create_table();

            table.cover_element(0);
            table.uncover_element(0);

            let second_header = table.nodes[4];
            assert_eq!(12, second_header.down);

            let bottom_node = table.nodes[21];
            assert_eq!(18, bottom_node.up);

            let third_header = table.nodes[7];
            assert_eq!(13, third_header.down);
        }
    }

    #[cfg(test)]
    mod cover_row {
        use super::*;
        
        #[test]
        fn element_headers_unchanged() {
            let mut table = create_table();
            table.cover_row(0, 1);

            let second_header = &table.nodes[4];
            assert_eq!(3, second_header.left);
            assert_eq!(5, second_header.right);

            let third_header = &table.nodes[7];
            assert_eq!(6, third_header.left);
            assert_eq!(0, third_header.right);
        }

        #[test]
        fn element_headers_disconnected() {
            let mut table = create_table();
            table.cover_row(0, 1);

            let before_second_header = &table.nodes[4];
            let after_second_header = &table.nodes[5];

            assert_eq!(5, before_second_header.right);
            assert_eq!(3, after_second_header.left);

            let before_third_header = &table.nodes[6];
            let after_third_header = &table.nodes[0];
            assert_eq!(0, before_third_header.right);
            assert_eq!(6, after_third_header.left);
        }
    }

    #[cfg(test)]
    mod uncover_row {
        use super::*;
        
        #[test]
        fn recovers_original_state() {
            let orig_table = create_table();
            let mut table = create_table();
            table.cover_row(0, 1);
            table.uncover_row(0, 1);

            assert_eq!(orig_table, table);
        }
    }
}



