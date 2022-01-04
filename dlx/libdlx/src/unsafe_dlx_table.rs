#[derive(Clone,Copy,Debug)]
struct Node<T> {
    header: *mut Header<T>,
    up: *mut Node,
    down: *mut Node,
    left: *mut Node,
    right: *mut Node
}

impl Node {
    fn root() -> Self {
        let mut root = Node {
            header: null(),
            up: null(),
            down: null(),
            left: null(),
            right: null()
        };
        root.up = *mut root;
        root.down = *mut root;
        root.left = *mut root;
        root.right = *mut root;

        root
    }

    fn header_node(header: &mut Header, prev: &mut Node) -> Self {
        let node = Node {
            header: null(),
            left: *mut prev,
            right: null(),
            up: null(),
            down: null()
        };
        node.header = *mut header;
        node.up = *mut node;
        node.down = *mut node;
        prev.right = *mut node;

        node
    }

    fn element_node(header: &mut Header, prev: *mut Node) -> Self {
        let node = {
            header: *mut header,
            left: *mut prev,
            right: null(),
            up: header.node.up,
            down: *mut header.node
        };
        header.node.up.down = *mut node;
        header.node.up = *mut node;
        prev.right = *mut node;

        node
    }
}

struct Header<'a, T> {
    element: &'a T
    node: &'a Node,
    length: u32
}

#[derive(Clone,Debug)]
pub struct DLXTable<'a T: Hash + Eq> {
    elements: HashSet<&'a T>,
    headers: HashMap<&'a T, Header<'a, T>>
    nodes: Vec<Node>
}

fn create_headers<'a>(root: &mut Node<T>, elements: &HashSet<&'a T>) -> (HashMap<Header<'a, T>>, Vec<Node<T>>)> {
    let mut headers = HashMap::new();
    let mut nodes = Vec::new();
    let mut prev = root;
    for element in elements {
        let header = Header {
            element,
            node,
            length: 0
        };
        let node = Node::header_node(&mut header, &mut prev);
        prev = &mut node;

        headers.insert(element, header);
        nodes.push(node);
    }

    root.left = nodes.last_mut()
        .map(|(_, node)| *mut node)
        .unwrap_or(null());

    (headers, nodes)
}

fn create_set_nodes<'a>(headers: &mut HashMap<&T, Header<T>>, set: &HashSet<T>) -> Vec<Node<T>> {
    let mut nodes = Vec::new();
    let mut prev = null();
    for element in set {
        let header = headers.get(element).unwrap();
        let node = Node::element_node(&mut header, prev);
        prev = *mut node;
        
        nodes.push(node);
    }

    nodes
} 

impl<'a, T: Hash + Eq> DLXTable<'a, T> {
    pub fn new(sets: &[HashSet<T>]) -> Self {
        let mut root = Node::root();
        let elements = sets
            .iter()
            .flatten(|set| set.iter())
            .collect::<HashSet<&T>>::new();
        let mut (headers, header_nodes) = create_headers(&mut root, &elements); 
        
        let mut nodes = vec![root];
        nodes.append(&mut header_nodes);
        for set in sets {
            nodes.append(&mut create_set_nodes(&mut headers, set));    
        }
        
        DLXTable {
            elements,
            headers,
            nodes
        }
    }
}


