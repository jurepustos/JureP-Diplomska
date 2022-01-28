use std::hint;
use std::ptr::null;
use Node::HeaderNode;
use Node::RootNode;
use Node::SpacerNode;
use Node::ItemNode;

#[derive(Clone,Copy,PartialEq,Eq)]
enum Node {
    RootNode(Root),
    HeaderNode(Header),
    SpacerNode(Spacer),
    ItemNode(Item)
}

#[derive(Clone,Copy,PartialEq,Eq)]
struct Root {
    prev: usize,
    next: usize
}

#[derive(Clone,Copy,PartialEq,Eq)]
struct Header {
    value: usize,
    prev: usize,
    next: usize,
    first: usize,
    last: usize,
    length: usize
}

#[derive(Clone,Copy,PartialEq,Eq)]
struct Item {
    header: usize,
    above: usize,
    below: usize
}

#[derive(Clone,Copy,PartialEq,Eq)]
struct Spacer {
    prev: usize,
    next: usize
}

impl Node {
    fn get_root(&self) -> Option<&Root> {
        match self {
            RootNode(root) => Some(root),
            _ => None
        }
    }

    fn get_root_mut(&mut self) -> Option<&mut Root> {
        match self {
            RootNode(ref mut root) => Some(root),
            _ => None
        }
    }

    fn get_header(&self) -> Option<&Header> {
        match self {
            HeaderNode(header) => Some(header),
            _ => None
        }
    }

    fn get_header_mut(&mut self) -> Option<&mut Header> {
        match self {
            HeaderNode(ref mut header) => Some(header),
            _ => None
        }
    }

    fn get_spacer(&self) -> Option<&Spacer> {
        match self {
            SpacerNode(spacer) => Some(spacer),
            _ => None
        }
    }

    fn get_spacer_mut(&mut self) -> Option<&mut Spacer> {
        match self {
            SpacerNode(ref mut spacer) => Some(spacer),
            _ => None
        }
    }

    fn get_item(&self) -> Option<&Item> {
        match self {
            ItemNode(item) => Some(item),
            _ => None
        }
    }

    fn get_item_mut(&mut self) -> Option<&mut Item> {
        match self {
            ItemNode(ref mut item) => Some(item),
            _ => None
        }
    }

    fn get_below(self) -> Option<usize> {
        match self {
            HeaderNode(header) => Some(header.first),
            ItemNode(item) => Some(item.below),
            SpacerNode(spacer) => Some(spacer.next),
            _ => None
        }
    }

    fn get_above(self) -> Option<usize> {
        match self {
            HeaderNode(header) => Some(header.last),
            ItemNode(item) => Some(item.above),
            SpacerNode(spacer) => Some(spacer.prev),
            _ => None
        }
    }

    fn get_prev(self) -> Option<usize> {
        match self {
            HeaderNode(header) => Some(header.prev),
            _ => None
        }
    }

    fn get_next(self) -> Option<usize> {
        match self {
            HeaderNode(header) => Some(header.next),
            _ => None
        }
    }

    fn set_above(&mut self, new_above: usize) {
        match self {
            ItemNode(item) => {
                item.above = new_above;
            },
            HeaderNode(header) => {
                header.last = new_above;
            }
            _ => {}
        }
    }

    fn set_below(&mut self, new_below: usize) {
        match self {
            ItemNode(item) => {
                item.below = new_below;
            },
            HeaderNode(header) => {
                header.first = new_below;
            }
            _ => {}
        }
    }
}

struct NodeList(Vec<Node>);

impl NodeList {
    fn get(&self, index: usize) -> Option<Node> {
        self.0.get(index).cloned()
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Node> {
        self.0.get_mut(index)
    }

    fn get_root(&self, index: usize) -> Option<Root> {
        match self.0.get(index) {
            Some(RootNode(root)) => Some(*root),
            _ => None
        }
    }

    fn get_root_mut(&mut self, index: usize) -> Option<&mut Root> {
        match self.0.get_mut(index) {
            Some(RootNode(ref mut root)) => Some(root),
            _ => None
        }
    }

    fn get_header(&self, index: usize) -> Option<Header> {
        match self.0.get(index) {
            Some(HeaderNode(header)) => Some(*header),
            _ => None
        }
    }

    fn get_header_mut(&mut self, index: usize) -> Option<&mut Header> {
        match self.0.get_mut(index) {
            Some(HeaderNode(ref mut header)) => Some(header),
            _ => None
        }
    }

    fn get_spacer(&self, index: usize) -> Option<Spacer> {
        match self.0.get(index) {
            Some(SpacerNode(spacer)) => Some(*spacer),
            _ => None
        }
    }

    fn get_spacer_mut(&mut self, index: usize) -> Option<&mut Spacer> {
        match self.0.get_mut(index) {
            Some(SpacerNode(ref mut spacer)) => Some(spacer),
            _ => None
        }
    }

    fn get_item(&self, index: usize) -> Option<Item> {
        match self.0.get(index) {
            Some(ItemNode(item)) => Some(*item),
            _ => None
        }
    }

    fn get_item_mut(&mut self, index: usize) -> Option<&mut Item> {
        match self.0.get_mut(index) {
            Some(ItemNode(ref mut item)) => Some(item),
            _ => None
        }
    }
}

pub struct DLXItem<'node> {
    index: usize,
    node: &'node Node
}

pub struct DLXTable {
    item_count: usize,
    nodes: NodeList
}

fn append_headers(nodes: &mut Vec<Node>, item_count: usize) {
    for val in 0..item_count {
        let node_index = val+1;
        nodes.push(HeaderNode(Header {
            value: val,
            prev: node_index-1,
            next: (node_index+1) % (item_count+1),
            first: node_index,
            last: node_index,
            length: 0
        }));
    }
}

fn append_sets(nodes: &mut Vec<Node>, sets: &Vec<Vec<usize>>) {
    let mut prev_spacer = 0;
    for set in sets {
        let start = nodes.len();
        nodes.push(SpacerNode(Spacer {
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

fn add_item_node(nodes: &mut Vec<Node>, current: usize, header_index: usize) {
    if let Some(HeaderNode(ref mut header)) = nodes.get_mut(header_index) {
        let above = header.last;
        if header.first == header_index {
            header.first = current;
        }
        header.last = current;
        header.length += 1;
        if let Some(ItemNode(ref mut prev_header)) = nodes.get_mut(current-1) {
            prev_header.below = current;
        }
        let item_node = ItemNode(Item {
            header: header_index,
            above,
            below: header_index
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
        nodes.push(RootNode(Root {
            prev: item_count,
            next: 1
        }));
        append_headers(&mut nodes, item_count);
        append_sets(&mut nodes, sets);

        DLXTable {
            item_count,
            nodes: NodeList(nodes)
        }
    }

    pub fn cover(&mut self, item: DLXItem) {
        if let Some(header) = self.nodes.get_header(item.index) {
            let mut index = header.first;
            while let Some(item) = self.nodes.get_item(index) {
                self.hide(index);
                index = item.below
            }

            let mut prev_header = self.nodes.get_header_mut(header.prev).unwrap();
            prev_header.next = header.next;

            let mut next_header = self.nodes.get_header_mut(header.next).unwrap();
            next_header.prev = header.prev;
        }
    }

    fn hide(&mut self, index: usize) {
        let mut q = index+1;
        while let Some(ItemNode(item)) = self.nodes.get(q) {
            self.nodes.get_mut(item.above).unwrap().set_below(item.below);
            self.nodes.get_mut(item.below).unwrap().set_above(item.above);
            let header = self.nodes.get_header_mut(item.header).unwrap();
            header.length -= 1;
            q += 1;
        }
    }

    pub fn uncover(&mut self, item: DLXItem) {
        if let Some(header) = self.nodes.get_header(item.index) {
            let prev = header.prev;
            let prev_header = self.nodes.get_header_mut(prev).unwrap();
            prev_header.next = item.index;

            let next = header.next;
            let next_header = self.nodes.get_header_mut(next).unwrap();
            next_header.prev = item.index;

            let mut index = header.first;
            while let Some(item) = self.nodes.get_item(index) {
                self.unhide(index);
                index = item.below;
            }
        }
    }

    fn unhide(&mut self, index: usize) {
        let mut q = index+1;
        while let Some(ItemNode(item)) = self.nodes.get(q) {
            self.nodes.get_mut(item.above).unwrap().set_below(q);
            self.nodes.get_mut(item.below).unwrap().set_above(q);
            let header = self.nodes.get_header_mut(item.header).unwrap();
            header.length += 1;
            q += 1;
        }
    }
}


