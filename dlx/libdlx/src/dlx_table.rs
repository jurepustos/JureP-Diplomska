use std::collections::btree_set::BTreeSet;
use std::collections::HashSet;
use std::iter::FromIterator;
use Node::HeaderNode;
use Node::SpacerNode;
use Node::ItemNode;

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
enum Node {
    HeaderNode(Header),
    SpacerNode(Spacer),
    ItemNode(Item)
}

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
struct ItemHeader {
    value: usize,
    prev: usize,
    next: usize
}

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
struct Header {
    first: usize,
    last: usize,
    length: usize
}

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
struct Item {
    header: usize,
    above: usize,
    below: usize
}

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
struct Spacer {
    prev: usize,
    next: usize
}

impl Node {
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
        }
    }

    fn get_above(self) -> Option<usize> {
        match self {
            HeaderNode(header) => Some(header.last),
            ItemNode(item) => Some(item.above),
            SpacerNode(spacer) => Some(spacer.prev),
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

#[derive(Clone,PartialEq,Eq,Debug)]
struct NodeList(Vec<Node>);

impl NodeList {
    fn get(&self, index: usize) -> Option<Node> {
        self.0.get(index).cloned()
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Node> {
        self.0.get_mut(index)
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


#[derive(Clone,PartialEq,Eq,Debug)]
pub struct DLXTable {
    item_headers: Vec<ItemHeader>,
    nodes: NodeList,
    primary_items_count: usize
}

fn make_root_header(item_count: usize) -> ItemHeader {
    if item_count > 0 {
        ItemHeader {
            value: usize::MAX,
            prev: item_count,
            next: 1
        }
    }
    else {
        ItemHeader {
            value: usize::MAX,
            prev: 0,
            next: 0
        }
    }
}

fn make_item_headers(item_count: usize) -> Vec<ItemHeader> {
    let mut item_headers = Vec::with_capacity(item_count+1);
    item_headers.push(make_root_header(item_count));
    for val in 0..item_count {
        let node_index = val+1;
        item_headers.push(ItemHeader {
            value: val,
            prev: node_index-1,
            next: (node_index+1) % (item_count+1)
        });
    }
    item_headers
}

fn append_headers(nodes: &mut Vec<Node>, item_count: usize) {
    for node_index in 0..item_count {
        nodes.push(HeaderNode(Header {
            first: node_index,
            last: node_index,
            length: 0
        }));
    }
}

fn append_sets(nodes: &mut Vec<Node>, sets: &Vec<Vec<usize>>) {
    let mut prev_spacer = usize::MAX;
    let nonempty_sets_iter =
        sets.iter().filter(|set| set.len() > 0);
    for set in nonempty_sets_iter {
        let start = nodes.len();
        add_set(nodes, prev_spacer, start, &set);
        prev_spacer = start+1;
    }
}

fn add_set(nodes: &mut Vec<Node>, prev_spacer: usize, start_index: usize, set: &[usize]) {
    nodes.push(SpacerNode(Spacer {
        prev: prev_spacer,
        next: start_index+set.len()
    }));
    for val in set {
        add_item_node(nodes, nodes.len(), *val);
    }
}

fn add_item_node(nodes: &mut Vec<Node>, current: usize, header_index: usize) {
    if let Some(HeaderNode(ref mut header)) = nodes.get_mut(header_index) {
        let above = header.last;
        if header.first == header_index {
            // No nodes for this item yet
            header.first = current;
        }
        header.last = current;
        header.length += 1;
        if let Some(ItemNode(above_item)) = nodes.get_mut(above) {
            above_item.below = current;
        }

        let item_node = ItemNode(Item {
            header: header_index,
            above,
            below: header_index
        });
        nodes.push(item_node);
    }
}

fn get_item_count(sets: &Vec<Vec<usize>>) -> usize {
    sets.iter()
        .flatten()
        .max()
        .map(|i| i+1)
        .unwrap_or(0)
}

fn get_item_instance_count(sets: &Vec<Vec<usize>>) -> usize {
    sets.iter()
        .flatten()
        .map(|_| 1)
        .sum::<usize>()
}

impl DLXTable {
    pub fn new(sets: Vec<Vec<usize>>, primary_items_count: usize) -> Self {
        let item_count = get_item_count(&sets);
        let node_count = get_item_instance_count(&sets) + 1 + item_count;

        let item_headers = make_item_headers(item_count);

        let mut nodes = Vec::with_capacity(node_count);
        append_headers(&mut nodes, item_count);
        append_sets(&mut nodes, &sets);

        DLXTable {
            item_headers,
            nodes: NodeList(nodes),
            primary_items_count
        }
    }

    pub fn cover(&mut self, val: usize) {
        if let Some(header) = self.nodes.get_header(val) {
            let mut i = header.first;
            while let Some(item) = self.nodes.get_item(i) {
                self.hide(i);
                i = item.below
            }

            let item_header = self.item_headers.get(val+1).cloned().unwrap();
            let prev = item_header.prev;
            let next = item_header.next;

            let mut prev_header = self.item_headers.get_mut(prev).unwrap();
            prev_header.next = next;

            let mut next_header = self.item_headers.get_mut(next).unwrap();
            next_header.prev = prev;
        }
    }

    fn hide(&mut self, index: usize) {
        let mut i = index+1;
        while i != index {
            match self.nodes.get(i) {
                Some(ItemNode(item)) => {
                    self.nodes.get_mut(item.above).unwrap().set_below(item.below);
                    self.nodes.get_mut(item.below).unwrap().set_above(item.above);
                    let header = self.nodes.get_header_mut(item.header).unwrap();
                    header.length -= 1;
                    i += 1;
                }
                Some(SpacerNode(spacer)) => {
                    i = spacer.prev;
                }
                _ => {
                    // We get here because there is no spacer at the end
                    break;
                }
            }
        }
    }

    pub fn uncover(&mut self, val: usize) {
        if let Some(item_header) = self.item_headers.get(val+1).cloned() {
            let prev = item_header.prev;
            let prev_header = self.item_headers.get_mut(prev).unwrap();
            prev_header.next = val+1;

            let next = item_header.next;
            let next_header = self.item_headers.get_mut(next).unwrap();
            next_header.prev = val+1;

            let header = self.nodes.get_header(val).unwrap();
            let mut i = header.last;
            while let Some(item) = self.nodes.get_item(i) {
                self.unhide(i);
                i = item.above;
            }
        }
    }

    fn unhide(&mut self, index: usize) {
        let mut i = index-1;
        while i != index {
            match self.nodes.get(i) {
                Some(ItemNode(item)) => {
                    self.nodes.get_mut(item.above).unwrap().set_below(i);
                    self.nodes.get_mut(item.below).unwrap().set_above(i);
                    let header = self.nodes.get_header_mut(item.header).unwrap();
                    header.length += 1;
                    i -= 1;
                }
                Some(SpacerNode(spacer)) => {
                    i = spacer.next;
                }
                _ => {
                    // No spacer at the end
                    break;
                }
            }
        }
    }

    pub fn get_current_items(&self) -> Vec<usize> {
        let mut items = Vec::new();
        let root = self.item_headers.get(0).unwrap();
        let mut i = root.next;
        while i != 0 {
            if let Some(item_header) = self.item_headers.get(i) {
                if i <= self.primary_items_count {
                    items.push(i-1);
                    i = item_header.next;
                }
                else {
                    break;
                }
            }
            else {
                return Vec::new();
            }
        }
        items
    }

    pub fn get_item_instances(&self, index: usize) -> Vec<usize> {
        let mut node_indices = Vec::new();
        if let Some(item) = self.nodes.get_item(index) {
            let header = self.nodes.get_header(item.header).unwrap();
            let mut i = header.first;
            while let Some(item) = self.nodes.get_item(i) {
                node_indices.push(i);
                i = item.below;
            }
        }
        node_indices
    }

    pub fn get_item_value(&self, index: usize) -> Option<usize> {
        self.nodes
            .get_item(index)
            .map(|item| self.item_headers.get(item.header+1))
            .flatten()
            .map(|item_header| item_header.value)
    }

    pub fn get_set_items(&self, index: usize) -> Vec<usize> {
        let mut set_items = vec![index];
        let mut i = index+1;
        while i != index {
            match self.nodes.get(i) {
                Some(ItemNode(item)) => {
                    set_items.push(i);
                    i += 1;
                },
                Some(SpacerNode(spacer)) => {
                    i = spacer.prev;
                },
                _ => {
                    break;
                }
            }
        }
        set_items
    }

    pub fn get_instance_count(&self, item: usize) -> usize {
        match self.nodes.get(item) {
            Some(ItemNode(item)) =>
                self.nodes
                    .get_header(item.header)
                    .map(|header| header.length)
                    .unwrap_or(0),
            Some(HeaderNode(header)) => {
                header.length
            },
            _ => 0
        }
    }

    pub fn cover_set(&mut self, index: usize) {
        let set_items = self.get_set_items(index);
        for set_item in set_items {
            if index != set_item {
                self.cover(self.get_item_value(set_item).unwrap());
            }
        }
    }

    pub fn uncover_set(&mut self, index: usize) {
        let set_items = self.get_set_items(index);
        for set_item in set_items.iter().rev() {
            if index != *set_item {
                self.uncover(self.get_item_value(*set_item).unwrap());
            }
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


// Tests
mod tests {
    use super::*;

    // 0234 124 34
    fn make_testing_table() -> DLXTable {
            DLXTable {
                item_headers: make_item_headers(5),
                nodes: NodeList(vec![
                    HeaderNode(Header {
                        first: 6,
                        last: 6,
                        length: 1
                    }),
                    HeaderNode(Header {
                        first: 11,
                        last: 11,
                        length: 1
                    }),
                    HeaderNode(Header {
                        first: 7,
                        last: 12,
                        length: 2
                    }),
                    HeaderNode(Header {
                        first: 8,
                        last: 15,
                        length: 2
                    }),
                    HeaderNode(Header {
                        first: 9,
                        last: 16,
                        length: 3
                    }),
                    SpacerNode(Spacer {
                        prev: usize::MAX,
                        next: 9
                    }),
                    ItemNode(Item {
                        header: 0,
                        above: 0,
                        below: 0
                    }),
                    ItemNode(Item {
                        header: 2,
                        above: 2,
                        below: 12
                    }),
                    ItemNode(Item {
                        header: 3,
                        above: 3,
                        below: 15
                    }),
                    ItemNode(Item {
                        header: 4,
                        above: 4,
                        below: 13
                    }),
                    SpacerNode(Spacer {
                        prev: 6,
                        next: 13
                    }),
                    ItemNode(Item {
                        header: 1,
                        above: 1,
                        below: 1
                    }),
                    ItemNode(Item {
                        header: 2,
                        above: 7,
                        below: 2
                    }),
                    ItemNode(Item {
                        header: 4,
                        above: 9,
                        below: 16
                    }),
                    SpacerNode(Spacer {
                        prev: 11,
                        next: 16
                    }),
                    ItemNode(Item {
                        header: 3,
                        above: 8,
                        below: 3
                    }),
                    ItemNode(Item {
                        header: 4,
                        above: 13,
                        below: 4
                    })
                ]),
                primary_items_count: 0
            }
        }

    #[cfg(test)]
    mod creation_tests {
        use super::super::*;
        use super::*;
        use Node::SpacerNode;
        use Node::HeaderNode;
        use Node::ItemNode;

        fn item_node_count(nodes: &[Node]) -> usize {
            nodes.iter()
                .filter(|node| node.get_item().is_some())
                .map(|_| 1)
                .sum()
        }

        fn spacer_count(nodes: &[Node]) -> usize {
            nodes.iter()
                .filter(|node| node.get_spacer().is_some())
                .map(|_| 1)
                .sum()
        }

        fn header_count(nodes: &[Node]) -> usize {
            nodes.iter()
                .filter(|node| node.get_header().is_some())
                .map(|_| 1)
                .sum()
        }

        fn assert_equal(table: DLXTable, expected: DLXTable) {
            assert_eq!(table.item_headers.len(), expected.item_headers.len());
            assert_eq!(header_count(&table.nodes.0), header_count(&expected.nodes.0));
            assert_eq!(spacer_count(&table.nodes.0), spacer_count(&expected.nodes.0));
            assert_eq!(item_node_count(&table.nodes.0), item_node_count(&expected.nodes.0));
            assert_eq!(table, expected);
        }

        #[test]
        fn empty() {
            let table = DLXTable::new(Vec::new(), 0);
            let expected = DLXTable {
                item_headers: make_item_headers(0),
                nodes: NodeList(Vec::new()),
                primary_items_count: 0
            };
            assert_equal(table, expected);
        }

        #[test]
        fn empty_set() {
            let table = DLXTable::new(vec![Vec::new()], 0);
            let expected = DLXTable {
                item_headers: make_item_headers(0),
                nodes: NodeList(Vec::new()),
                primary_items_count: 0
            };
            assert_equal(table, expected);
        }

        #[test]
        fn one_element() {
            let table = DLXTable::new(vec![vec![0]], 0);
            let expected = DLXTable {
                item_headers: make_item_headers(1),
                nodes: NodeList(vec![
                    HeaderNode(Header {
                        first: 2,
                        last: 2,
                        length: 1
                    }),
                    SpacerNode(Spacer {
                        prev: usize::MAX,
                        next: 2
                    }),
                    ItemNode(Item {
                        header: 0,
                        above: 0,
                        below: 0
                    })]),
                primary_items_count: 0
            };
            assert_equal(table, expected);
        }

        #[test]
        fn multiple_elements() {
            let table = DLXTable::new(vec![vec![0,1,2,3]], 0);
            let expected = DLXTable {
                item_headers: make_item_headers(4),
                nodes: NodeList(vec![
                    HeaderNode(Header {
                        first: 5,
                        last: 5,
                        length: 1
                    }),
                    HeaderNode(Header {
                        first: 6,
                        last: 6,
                        length: 1
                    }),
                    HeaderNode(Header {
                        first: 7,
                        last: 7,
                        length: 1
                    }),
                    HeaderNode(Header {
                        first: 8,
                        last: 8,
                        length: 1
                    }),
                    SpacerNode(Spacer {
                        prev: usize::MAX,
                        next: 8
                    }),
                    ItemNode(Item {
                        header: 0,
                        above: 0,
                        below: 0
                    }),
                    ItemNode(Item {
                        header: 1,
                        above: 1,
                        below: 1
                    }),
                    ItemNode(Item {
                        header: 2,
                        above: 2,
                        below: 2
                    }),
                    ItemNode(Item {
                        header: 3,
                        above: 3,
                        below: 3
                    })
                ]),
                primary_items_count: 0
            };
            assert_equal(table, expected);
        }

        #[test]
        fn disjoint_test() {
            let table = DLXTable::new(vec![vec![0,1,2], vec![3,4]], 0);
            let expected = DLXTable {
                item_headers: make_item_headers(5),
                nodes: NodeList(vec![
                    HeaderNode(Header {
                        first: 6,
                        last: 6,
                        length: 1
                    }),
                    HeaderNode(Header {
                        first: 7,
                        last: 7,
                        length: 1
                    }),
                    HeaderNode(Header {
                        first: 8,
                        last: 8,
                        length: 1
                    }),
                    HeaderNode(Header {
                        first: 10,
                        last: 10,
                        length: 1
                    }),
                    HeaderNode(Header {
                        first: 11,
                        last: 11,
                        length: 1
                    }),
                    SpacerNode(Spacer {
                        prev: usize::MAX,
                        next: 8
                    }),
                    ItemNode(Item {
                        header: 0,
                        above: 0,
                        below: 0
                    }),
                    ItemNode(Item {
                        header: 1,
                        above: 1,
                        below: 1
                    }),
                    ItemNode(Item {
                        header: 2,
                        above: 2,
                        below: 2
                    }),
                    SpacerNode(Spacer {
                        prev: 6,
                        next: 11
                    }),
                    ItemNode(Item {
                        header: 3,
                        above: 3,
                        below: 3
                    }),
                    ItemNode(Item {
                        header: 4,
                        above: 4,
                        below: 4
                    })
                ]),
                primary_items_count: 0
            };
            assert_equal(table, expected);
        }

        #[test]
        fn overlapping_sets() {
            let table = DLXTable::new(vec![vec![0,2,3,4], vec![1,2,4], vec![3,4]], 0);
            let expected = make_testing_table();
            assert_equal(table, expected);
        }
    }

    #[cfg(test)]
    mod cover_tests {
        use super::*;
        use super::super::*;

        fn assert_covered_header(table: &DLXTable, index: usize) {
            let prev_index = if index >= 1 {
                    index-1
                }
                else {
                    table.item_headers.len()-1
                };
            let next_index = if index < table.item_headers.len()-1 {
                    index+1
                }
                else {
                    0
                };
            let prev_header = table.item_headers[prev_index];
            let header = table.item_headers[index];
            let next_header = table.item_headers[next_index];

            assert_eq!(prev_header.next, next_index);
            assert_eq!(next_header.prev, prev_index);
            assert_eq!(header.prev, prev_index);
            assert_eq!(header.next, next_index);
        }

        #[test]
        fn test_cover() {
            // 0234 124 34
            let mut table = make_testing_table();
            table.cover(1);

            assert_covered_header(&table, 2);
            assert_eq!(table.nodes.get_header(2).unwrap().first, 7);
            assert_eq!(table.nodes.get_header(2).unwrap().last, 7);

            assert_eq!(table.nodes.get_header(4).unwrap().first, 9);
            assert_eq!(table.nodes.get_header(4).unwrap().last, 16);

            assert_eq!(table.nodes.get_item(7).unwrap().below, 2);
            assert_eq!(table.nodes.get_item(7).unwrap().above, 2);

            assert_eq!(table.nodes.get_item(9).unwrap().below, 16);
            assert_eq!(table.nodes.get_item(9).unwrap().above, 4);

            assert_eq!(table.nodes.get_item(12).unwrap().below, 2);
            assert_eq!(table.nodes.get_item(12).unwrap().above, 7);

            assert_eq!(table.nodes.get_item(13).unwrap().below, 16);
            assert_eq!(table.nodes.get_item(13).unwrap().above, 9);

        }

        #[test]
        fn test_uncover() {
            let original_table = make_testing_table();
            let mut table = make_testing_table();
            table.cover(1);
            table.uncover(1);
            assert_eq!(table, original_table);
        }
    }
}



