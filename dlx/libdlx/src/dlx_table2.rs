use Node::Header;
use Node::Root;
use Node::Spacer;
use Node::Item;

#[derive(Clone,Copy,PartialEq,Eq)]
enum Node {
    Root(RootNode),
    Header(HeaderNode),
    Spacer(SpacerNode),
    Item(ItemNode)
}

#[derive(Clone,Copy,PartialEq,Eq)]
struct RootNode {
    prev: usize,
    next: usize
}

#[derive(Clone,Copy,PartialEq,Eq)]
struct HeaderNode {
    value: usize,
    prev: usize,
    next: usize,
    first: Option<usize>,
    last: Option<usize>,
    length: usize
}

#[derive(Clone,Copy,PartialEq,Eq)]
struct ItemNode {
    header: usize,
    above: usize,
    below: usize
}

#[derive(Clone,Copy,PartialEq,Eq)]
struct SpacerNode {
    prev: usize,
    next: usize
}

struct DLXTable {
    item_count: usize,
    nodes: Vec<Node>
}

fn append_headers(nodes: &mut Vec<Node>, item_count: usize) {
    for val in 0..item_count {
        let node_index = val+1;
        nodes.push(Header(HeaderNode {
            value: val,
            prev: node_index-1,
            next: (node_index+1) % (item_count+1),
            first: Some(node_index),
            last: Some(node_index),
            length: 0
        }));
    }
}

fn append_sets(nodes: &mut Vec<Node>, sets: &Vec<Vec<usize>>) {
    let mut prev_spacer = 0;
    for set in sets {
        let start = nodes.len();
        nodes.push(Spacer(SpacerNode {
            prev: prev_spacer,
            next: start+set.len()
        }));
        prev_spacer = nodes.len();
        add_set(nodes, &set);
    }
}

fn add_set(nodes: &mut Vec<Node>, set: &[usize]) {
    for val in set {
        let current = nodes.len();
        let header = val+1;
        add_item_node(nodes, current, header);
    }
}

fn add_item_node(nodes: &mut Vec<Node>, current: usize, header: usize) {
    if let Some(Header(ref mut header_node)) = nodes.get_mut(header) {
        let above = header_node.last.unwrap_or(header);
        if header_node.first.is_none() {
            header_node.first = Some(current);
        }
        header_node.last = Some(current);
        header_node.length += 1;
        if let Some(Item(ref mut prev_node)) = nodes.get_mut(current-1) {
            prev_node.below = current;
        }
        let item_node = Item(ItemNode {
            header,
            above,
            below: header
        });
        nodes.push(item_node);
    }
}

impl DLXTable {
    pub fn new(sets: &Vec<Vec<usize>>) -> Self {
        let item_count =
            *sets
            .iter()
            .flatten()
            .max()
            .unwrap_or(&0);

        let node_count = sets
            .iter()
            .flatten()
            .map(|_| 1)
            .sum::<usize>() + 1 + item_count;

        let mut nodes = Vec::with_capacity(node_count);
        nodes.push(Root(RootNode {
            prev: item_count,
            next: 1
        }));
        append_headers(&mut nodes, item_count);
        append_sets(&mut nodes, sets);

        DLXTable {
            item_count,
            nodes
        }
    }


}


