use std::collections::HashSet;

pub type Element = usize;
pub enum Node<'a> {
    Root {
        first: Option<&'a mut Node<'a>>,
        last: Option<&'a mut Node<'a>>
    },
    Header { 
        element: Element,
        up: Option<&'a mut Node<'a>>,
        down: Option<&'a mut Node<'a>>,
        left: &'a mut Node<'a>,
        right: &'a mut Node<'a>
    },
    Element {
        header: &'a mut Node<'a>,
        up: &'a mut Node<'a>,
        down: &'a mut Node<'a>
    },
    Spacer {
        prev: Option<&'a mut Node<'a>>,
        next: Option<&'a mut Node<'a>>
    }
}

impl<'a> Node<'a> {
    fn is_header_for(&self, elem: Element) -> bool {
        match &self {
            Node::Header{ element, .. } => 
                *element == elem,
            other => false
        }
    }
}

pub struct DLXTable<'a>(Vec<Node<'a>>);
struct HeaderNodes<'a> { nodes: Vec<Node<'a>> }

impl<'a> HeaderNodes<'a> {
    fn elem_node(&'a self, elem: Element) -> Option<&'a mut Node<'a>> {
        self.nodes.iter_mut()
            .filter(|node|
                node.is_header_for(elem))
            .next()
    }
}

impl<'a> DLXTable<'a> {
    pub fn from(sets: &'a Vec<Vec<Element>>) -> Self {
        let mut nodes: Vec<Node<'a>> = Vec::with_capacity(sets.len());
        nodes.push(Node::Root { first: None, last: None });
        let mut root = &mut nodes.get_mut(0).unwrap();
        let headers = table_headers(&sets, &mut root);
        for header in headers.nodes {
            nodes.push(header);
        }
        let mut header_spacer = Node::Spacer {
            prev: None,
            next: None
        };
        nodes.push(header_spacer);

        let spacer_index = nodes.len() - 1;
        for set in sets {
            let mut last_spacer = nodes.get_mut(spacer_index).unwrap();
            let row_nodes = set_nodes(&set, &mut headers, &mut last_spacer);
            for node in row_nodes {
                nodes.push(node);
            }

            if let Some(first_node) = nodes.get_mut(spacer_index+1) {
                let next_spacer = Node::Spacer {
                    prev: Some(first_node), 
                    next: None
                };
                nodes.push(next_spacer);
                spacer_index = nodes.len() - 1;
            }
        }

        DLXTable(nodes)
    }
}

fn table_headers<'a>(sets: &'a Vec<Vec<Element>>, root: &'a mut Node<'a>) -> HeaderNodes<'a> {
    let mut nodes: Vec<Node> = Vec::new();
    let elements: HashSet<Element> = 
        sets.iter()
            .flat_map(|set| set.iter())
            .map(|&elem| elem)
            .collect();

    for element in elements {
        let mut prev_node = 
            match nodes.last_mut() {
                None => root,
                Some(prev_header) => prev_header
            };
        let new_node = Node::Header {
            element,
            up: None,
            down: None,
            left: &mut prev_node,
            right: &mut root
        };
        match prev_node {
            Node::Root{ mut first, mut last } => {
                first = Some(&mut new_node);
                last = Some(&mut new_node);
            },
            Node::Header{ mut right, mut left, .. } => {
                right = &mut new_node;
                left = &mut new_node;
            }
        };
        nodes.push(new_node);
    }

    HeaderNodes { nodes: nodes }
}


fn set_nodes<'a>(set: &Vec<Element>, headers: &'a mut HeaderNodes<'a>, last_spacer: &'a mut Node<'a>) -> Vec<Node<'a>> {
    let mut nodes: Vec<Node> = Vec::new();
    for &elem in set {
        if let Some(header_node) = headers.elem_node(elem) {
            if let Node::Header{mut up,..} = header_node {
                let mut up_node = match up {
                    None => &mut header_node,
                    Some(last_node) => &mut last_node
                };
                let new_node = Node::Element {
                    header: &mut header_node,
                    up: &mut up_node,
                    down: &mut header_node
                };
                if let Node::Element{mut down,..} = up_node {
                    down = &mut new_node;
                }
                nodes.push(new_node);
            }
        }
    }
    if let Node::Spacer{mut next,..} = last_spacer {
        next = nodes.get_mut(0);
    }
    
    nodes
}
