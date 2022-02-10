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
struct ItemHeader {
    value: usize,
    prev: usize,
    next: usize
}

#[derive(Clone,Copy,PartialEq,Eq)]
struct Header {
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

pub struct DLXTable {
    item_count: usize,
    item_headers: Vec<ItemHeader>,
    nodes: NodeList
}

fn append_headers(nodes: &mut Vec<Node>, item_count: usize) {
    for val in 0..item_count {
        let node_index = val+1;
        nodes.push(HeaderNode(Header {
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
    pub fn new(sets: Vec<Vec<usize>>) -> Self {
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

        let mut item_headers = Vec::with_capacity(item_count+1);
        item_headers.push(ItemHeader {
            value: 0,
            prev: item_count,
            next: 1
        });
        for val in 0..item_count {
            let node_index = val+1;
            item_headers.push(ItemHeader {
                value: val,
                prev: node_index-1,
                next: (node_index+1) % (item_count+1)
            });
        }

        let mut nodes = Vec::with_capacity(node_count);
        nodes.push(RootNode(Root {
            prev: item_count,
            next: 1
        }));
        append_headers(&mut nodes, item_count);
        append_sets(&mut nodes, &sets);

        DLXTable {
            item_count,
            item_headers,
            nodes: NodeList(nodes)
        }
    }

    pub fn cover(&mut self, index: usize) {
        if let Some(header) = self.nodes.get_header(index) {
            let mut i = header.first;
            while let Some(item) = self.nodes.get_item(i) {
                self.hide(index);
                i = item.below
            }

            let item_header = self.item_headers.get(index).cloned().unwrap();

            let mut prev_header = self.item_headers.get_mut(item_header.prev).unwrap();
            prev_header.next = item_header.next;

            let mut next_header = self.item_headers.get_mut(item_header.next).unwrap();
            next_header.prev = item_header.prev;
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

    pub fn uncover(&mut self, index: usize) {
        if let Some(item_header) = self.item_headers.get(index).cloned() {
            let prev = item_header.prev;
            let prev_header = self.item_headers.get_mut(prev).unwrap();
            prev_header.next = index;

            let next = item_header.next;
            let next_header = self.item_headers.get_mut(next).unwrap();
            next_header.prev = index;

            let header = self.nodes.get_header(index).unwrap();
            let mut i = header.first;
            while let Some(item) = self.nodes.get_item(i) {
                self.unhide(index);
                i = item.below;
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

    pub fn get_current_items(&self) -> Vec<usize> {
        let mut headers = Vec::new();
        let root = self.nodes.get_root(0).unwrap();
        let mut i = root.next;
        while let Some(_) = self.nodes.get_header(i) {
            headers.push(i);
            i += 1;
        }
        headers
    }

    pub fn get_item_instances(&self, index: usize) -> Vec<usize> {
        let mut items = Vec::new();
        let mut i = index;
        while let Some(_) = self.nodes.get_item(i) {
            items.push(i);
            i += 1;
        }
        items
    }

    pub fn get_set_items(&self, index: usize) -> Vec<usize> {
        let mut set_items = vec![index];
        let mut i = index+1;
        while i != index {
            let node = self.nodes.get(i);
            match node {
                Some(ItemNode(_)) => {
                    set_items.push(i);
                    i += 1;
                },
                Some(SpacerNode(spacer)) => {
                    i = spacer.prev;
                },
                _ => {
                    return Vec::new();
                }
            };
        }
        set_items
    }

    pub fn get_instance_count(&self, index: usize) -> usize {
        match self.nodes.get(index) {
            Some(ItemNode(item)) => {
                self.nodes
                    .get_header(item.header)
                    .map(|header| header.length)
                    .unwrap_or(0)
            },
            Some(HeaderNode(header)) => {
                header.length
            },
            _ => 0
        }
    }

    pub fn cover_set(&mut self, item: usize) {
        let set_items = self.get_set_items(item);
        for set_item in set_items {
            self.cover(set_item);
        }
    }

    pub fn uncover_set(&mut self, item: usize) {
        let set_items = self.get_set_items(item);
        for set_item in set_items {
            self.uncover(set_item);
        }
    }

    pub fn get_next_instance(&self, index: usize) -> Option<usize> {
        match self.nodes.get(index) {
            Some(HeaderNode(header)) =>
                self.nodes.get_item(header.first).and(Some(header.first)),
            Some(ItemNode(item)) =>
                self.nodes.get_item(item.below).and(Some(item.below)),
            _ => None
        }
    }
}




