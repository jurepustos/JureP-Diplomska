use std::collections::HashSet;
use std::hash::Hash;
use std::collections::HashMap;
use std::ptr::{null, null_mut};

pub struct Item<T> {
    name: T,
    first_node: *mut Node<T>,
    last_node: *mut Node<T>,
    left: *mut Item<T>,
    right: *mut Item<T>,
    count: u32
}

impl<T: Copy> Item<T> {
    pub fn new(name: T, prev_item: *mut Item<T>) -> Self {
        let mut item = Item {
            name,
            first_node: null_mut(),
            last_node: null_mut(),
            left: prev_item,
            right: null_mut(),
            count: 0
        };
        unsafe { item.link(); }
        item
    }

    unsafe fn link(&mut self) {
        if !self.left.is_null() {
            (*self.left).right = self;
        }
    }
}

struct Node<T> {
    item: *mut Item<T>,
    left: *mut Node<T>,
    right: *mut Node<T>,
    up: *mut Node<T>,
    down: *mut Node<T>
}

impl<T> Node<T> {
    pub fn new(item: &mut Item<T>, prev_node: *mut Node<T>) -> Self {
        let up = item.last_node;
        let down = item.first_node;
        let mut node = Node {
            item,
            left: prev_node,
            right: null_mut(),
            up,
            down
        };
        unsafe { node.link(); }
        node
    }

    unsafe fn link(&mut self) {
        (*self.item).last_node = self;
        (*self.item).count += 1;
        (*self.up).down = self;
        (*self.down).up = self;
        if !self.left.is_null() {
            (*self.left).right = self;
        }
    }
}

pub struct DLXTable<T: Hash + Eq + Copy> {
    items: Vec<Item<T>>,
    nodes: Vec<Node<T>>
}

fn create_items<T: Hash + Eq + Copy>(values: &[T]) -> Vec<Item<T>> {
    let mut items = Vec::new();
    let mut first_item = null_mut();
    let mut prev_item = null_mut();
    for v in values {
        let mut item = Item::new(*v, prev_item);
        if prev_item.is_null() {
            first_item = &mut item;
        }
        prev_item = &mut item;
        items.push(item);
    }

    let last_item = prev_item;
    unsafe {
        (*first_item).left = prev_item;
        (*last_item).right = first_item;
    }

    items
}

unsafe fn create_set_nodes<T: Hash + Eq>(set: &HashSet<T>, item_map: &HashMap<T,*mut Item<T>>) -> Vec<Node<T>> {
    let mut nodes = Vec::new();
    let mut prev_node = null_mut();
    for v in set {
        if let Some(item) = item_map.get(v) {
            let mut node = Node::new(&mut **item, prev_node);
            prev_node = &mut node;
            nodes.push(node);
        }
    }

    nodes
}

unsafe fn create_nodes<T: Hash + Eq>(sets: &[HashSet<T>], item_map: &HashMap<T,*mut Item<T>>) -> Vec<Node<T>> {
    let mut nodes = Vec::new();
    for set in sets {
        nodes.append(&mut create_set_nodes(set, item_map));
    }
    nodes
}

impl<T: Hash + Eq + Copy> DLXTable<T> {
    pub fn from(sets: &[HashSet<T>]) -> Self {
        let set_items: Vec<T> = sets
            .iter()
            .flatten()
            .map(|v| *v)
            .collect();
        let mut items = create_items(&set_items);
        let item_map: HashMap<T,*mut Item<T>> = items
            .iter_mut()
            .map(|item| (item.name, item as *mut Item<T>))
            .collect();
        let nodes = unsafe { create_nodes(sets, &item_map) };

        DLXTable {
            items,
            nodes
        }
    }
}


mod tests {
    #[cfg(test)]
    mod creation {

    }
}

