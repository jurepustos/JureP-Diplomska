use std::hash::Hash;
use std::collections::{HashSet, HashMap};
use std::ptr::{null, null_mut};

#[derive(Clone,Copy,Debug)]
struct Node<T> {
    header: *mut Header<T>,
    up: *mut Node<T>,
    down: *mut Node<T>,
    left: *mut Node<T>,
    right: *mut Node<T>
}

impl<T> Node<T> {
    fn root() -> Self {
        let mut root = Node {
            header: null_mut(),
            up: null_mut(),
            down: null_mut(),
            left: null_mut(),
            right: null_mut()
        };
        root.up = &mut root;
        root.down = &mut root;
        root.left = &mut root;
        root.right = &mut root;

        root
    }

    fn header_node(header: &mut Header<T>, prev: *mut Node<T>) -> Self {
        let mut node = Node {
            header,
            left: prev,
            right: null_mut(),
            up: null_mut(),
            down: null_mut()
        };
        node.up = &mut node;
        node.down = &mut node;
        unsafe {
            (*prev).right = &mut node;
        }

        node
    }

    fn element_node(header: &mut Header<T>, prev: *mut Node<T>) -> Self {
        unsafe {
            let mut node = Node {
                header,
                left: prev,
                right: null_mut(),
                up: (*header.node).up,
                down: header.node
            };
            (*(*header.node).up).down = &mut node;
            (*header.node).up = &mut node;
            (*prev).right = &mut node;

            node
        }
    }
}

struct Header<T> {
    name: T,
    node: *mut Node<T>,
    length: u32
}

pub struct DLXTable<T: Hash + Eq + Copy> {
    headers: HashMap<T, Header<T>>,
    nodes: Vec<Node<T>>
}

fn create_headers<T: Hash + Eq + Copy>(root: &mut Node<T>, elements: &HashSet<T>) -> (HashMap<T, Header<T>>, Vec<Node<T>>) {
    let mut headers = HashMap::new();
    let mut nodes = Vec::new();
    let mut prev: *mut Node<T> = root;
    for element in elements {
        let mut header = Header {
            name: *element,
            node: null_mut(),
            length: 0
        };

        let mut node = Node::header_node(&mut header, prev);
        header.node = &mut node;
        prev = &mut node;
        nodes.push(node);


        headers.insert(*element, header);
    }

    root.left = nodes.last_mut()
        .map(|node| node as *mut Node<T>)
        .unwrap_or(null_mut());

    (headers, nodes)
}

fn create_set_nodes<T: Hash + Eq>(headers: &mut HashMap<T, Header<T>>, set: &HashSet<T>) -> Vec<Node<T>> {
    let mut nodes = Vec::new();
    let mut prev = null_mut();
    for element in set {
        if let Some(mut header) = headers.get_mut(element) {
            let mut node = Node::element_node(&mut header, prev);
            prev = &mut node;
            nodes.push(node);
        }
    }

    nodes
} 

impl<T: Hash + Eq + Copy> DLXTable<T> {
    pub fn new(sets: &[HashSet<T>]) -> Self {
        let mut root = Node::root();
        let elements = sets
            .iter()
            .flatten()
            .map(|v| *v)
            .collect::<HashSet<T>>();

        let (mut headers, mut header_nodes) =
            create_headers(&mut root, &elements);
        
        let mut nodes = vec![root];
        nodes.append(&mut header_nodes);
        for set in sets {
            nodes.append(&mut create_set_nodes(&mut headers, set));    
        }
        
        DLXTable {
            headers,
            nodes
        }
    }
}


