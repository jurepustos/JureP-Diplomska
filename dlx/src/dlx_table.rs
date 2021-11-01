use std::collections::HashSet;

#[derive(Clone,Copy,PartialEq,Eq)]
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

#[derive(Clone,PartialEq,Eq)]
struct DLXTable {
    elements: Vec<String>,
    nodes: Vec<Node>
}


impl DLXTable {
    fn new() -> Self {
        DLXTable { 
            elements: vec![], 
            nodes: vec![Node::root()] 
        }
    }

    fn with_capacity(elements: Vec<String>, sets_count: usize) -> Self {
        let mut nodes = Vec::with_capacity(elements.len()+sets_count+1); 
        nodes.push(Node::root());
        
        DLXTable { 
            // headers: Vec::with_capacity(sets_count),
            elements, 
            nodes: nodes
        }
    }

    pub fn from(sets: &[Vec<&str>]) -> Self {
        let elements: Vec<String> = set_elements(&sets);
        let mut table = Self::with_capacity(elements, sets.len());
        table.create_headers();
        for set in sets {
            table.add_set(&set);
        }

        table
    }

    fn create_headers(&mut self) {
        let length = self.elements.len();
        for i in 0..length {
            self.nodes.push(Node::header(i+1, length));
            let elem = self.elements[i].to_owned();
            // self.headers.push(Header::new(elem, i+1));
        }
        let mut root = &mut self.nodes[0];
        root.left = length;
    }

    fn add_set(&mut self, set: &[&str]) {
        let offset = self.nodes.len();
        let elem_count = set.len();
        let last_index = elem_count - 1;
        let mut prev_index = last_index;
        for (current_index, &elem) in set.iter().enumerate() {
            let op_index = self.elements.iter()
                .position(|element| element == elem);
            
            if let Some(elem_index) = op_index {
                // let mut header = &mut self.headers[elem_index]; 
                // header.len += 1;

                let header_index = elem_index+1;
                let mut header_node = &mut self.nodes[header_index];
                let up_index = header_node.up;

                self.nodes.push(Node::element(header_index, header_node.up, 
                    offset, prev_index, current_index, elem_count));

                let mut up_node = &mut self.nodes[up_index];
                up_node.down = offset + current_index; 
                header_node.up = offset + current_index;
                header_node.len += 1;
                prev_index = current_index;
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
            let mut left_node = &mut self.nodes[header_node.left];
            let mut right_node = &mut self.nodes[header_node.right];
            left_node.right = header_node.right;
            right_node.left = header_node.left;
        }
    }

    fn element_nodes(&mut self, header_index: usize) -> Vec<usize> {
        let mut indices = Vec::new();
        if header_index <= self.elements.len() {
            let mut next_node = &self.nodes[header_index];
            while next_node.down != header_index {
                indices.push(next_node.down);
                next_node = &mut self.nodes[next_node.down];
            }
        }

        indices
    }

    fn row_nodes(&mut self, node_index: usize) -> Vec<usize> {
        let mut indices = Vec::new();
        if let Some(start_node) = self.nodes.get_mut(node_index) {
            let mut next_node = start_node;
            while next_node.right != node_index {
                indices.push(next_node.right);
                next_node = &mut self.nodes[next_node.right];
            }
        }

        indices
    }

    fn hide_row(&mut self, node_index: usize) {
        for index in self.row_nodes(node_index) {
            let node = &self.nodes[index];
            let mut up_node = &mut self.nodes[node.up];
            let mut down_node = &mut self.nodes[node.down];
            let mut header_node = &mut self.nodes[node.header];
            up_node.down = node.down;
            down_node.up = node.up;
            header_node.len -= 1;
        }
    }

    pub fn cover_row(&mut self, node_index: usize) {
        for index in self.row_nodes(node_index) {
            let node = &self.nodes[index];
            self.cover_element(node.header-1);
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
            let mut header_node = &mut self.nodes[header_index];
            let mut left_node = &mut self.nodes[header_node.left];
            let mut right_node = &mut self.nodes[header_node.right];

            left_node.right = header_index;
            right_node.left = header_index;
        }
    }

    fn unhide_row(&mut self, node_index: usize) {
        for &index in self.row_nodes(node_index).iter().rev() {
            let mut node = &mut self.nodes[index];
            let mut up_node = &mut self.nodes[node.up];
            let mut down_node = &mut self.nodes[node.down];
            let mut header_node = &mut self.nodes[node.header];

            up_node.down = node_index;
            down_node.up = node_index;
            header_node.len += 1;
        }
    }

    pub fn uncover_row(&mut self, node_index: usize) {
        for &index in self.row_nodes(node_index).iter().rev() {
            let node = &self.nodes[index];
            self.uncover_element(node.header-1);
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

    elements.into_iter()
        .map(|elem| elem.to_string())
        .collect()
}


