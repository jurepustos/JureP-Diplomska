use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::{Index, IndexMut};

struct OffsetVec<T> {
    base: Vec<T>,
    offset: usize
}

impl<T> OffsetVec<T> {
    fn new(offset: usize) -> Self {
        OffsetVec {
            base: Vec::new(),
            offset
        }
    }

    fn from(base: Vec<T>, offset: usize) -> Self {
        OffsetVec {
            base,
            offset
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index < self.offset {
            None
        }
        else {
            self.base.get(index-self.offset)
        }
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.offset {
            None
        }
        else {
            self.base.get_mut(index-self.offset)
        }
    }
}

impl<T> Index<usize> for OffsetVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.base.index(index - self.offset)
    }
}

impl<T> IndexMut<usize> for OffsetVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.base.index_mut(index - self.offset)
    }
}

struct DLXTable<'a, T> {
    llinks: Vec<usize>,
    rlinks: Vec<usize>,
    ulinks: OffsetVec<usize>,
    dlinks: Vec<usize>,
    tops: OffsetVec<usize>,
    names: OffsetVec<&'a T>,
    lengths: OffsetVec<usize>
}

impl<'a, T: Eq + Hash> DLXTable<'a, T> {
    fn new(sets: &'a Vec<Vec<T>>) -> Self {
        let items = sets
            .iter()
            .flatten()
            .collect::<HashSet<&T>>()
            .into_iter()
            .collect::<Vec<&T>>();

        let item_indices = items
            .iter()
            .enumerate()
            .map(|(i, item)| (*item, i))
            .collect::<HashMap<&T,usize>>();

        let item_count = items.len();
        let mut llinks = vec![0; item_count+1];
        llinks.insert(0, item_count);
        for i in 0..item_count {
            llinks.insert(i+1, i);
        }

        let mut rlinks = vec![0; item_count+1];
        rlinks.insert(0, 1);
        for i in 0..item_count-1 {
            rlinks.insert(i+1, i+2);
        }
        rlinks.insert(item_count+1, 0);

        let names = OffsetVec {
            base: items,
            offset: 1
        };

        let mut lengths = OffsetVec::from(vec![0; item_count], 1);

        let node_count = sets
            .iter()
            .map(|set| set.len())
            .sum::<usize>() + sets.len();

        let mut tops = OffsetVec::from(vec![0; node_count], 8);
        let mut ulinks = OffsetVec::from(vec![0; item_count+node_count-1], 1);
        let mut dlinks = vec![0; item_count+node_count+1];

        let mut i = item_count+1;
        let mut prev_spacer_index = i;
        for set in sets {
            i += 1;
            let mut iter = set.iter();
            while let Some(Some(item_index)) = iter
                .next()
                .map(|t| item_indices.get(t)) {
                tops[i] = *item_index;
                ulinks[i] = ulinks[*item_index];
                dlinks[ulinks[*item_index]] = i;
                ulinks[*item_index] = i;
                dlinks[i] = *item_index;
                i += 1;
            }
            dlinks[prev_spacer_index] = i-1;
            ulinks[i] = prev_spacer_index+1;
            prev_spacer_index = i;
        }

        DLXTable {
            llinks,
            rlinks,
            ulinks,
            dlinks,
            tops,
            names,
            lengths
        }
    }

    fn cover(&mut self, i: usize) {
        let mut p = self.dlinks[i];
        while p != i {
            self.hide(p);
            p = self.dlinks[p];
        }
        let l = self.llinks[i];
        let r = self.dlinks[i];
        self.rlinks[l] = r;
        self.llinks[r] = l;
    }

    fn hide(&mut self, p: usize) {
        let mut q = p+1;
        while q != p {
            let x = self.tops[q];
            let u = self.ulinks[q];
            let d = self.dlinks[q];
            if x == 0 {
                q = u;
            }
            else {
                self.dlinks[u] = d;
                self.ulinks[d] = u;
                self.lengths[x] -= 1;
                q += 1;
            }
        }
    }

    fn uncover(&mut self, i: usize) {
        let l = self.llinks[i];
        let r = self.rlinks[i];
        self.rlinks[l] = i;
        self.llinks[r] = i;
        let mut p = self.ulinks[i];
        while p != i {
            self.unhide(p);
             p = self.ulinks[p];
        }
    }

    fn unhide(&mut self, p: usize) {
        let mut q = p-1;
        while q != p {
            let x = self.tops[q];
            let u = self.ulinks[q];
            let d = self.dlinks[q];
            if x == 0 {
                q = d;
            }
            else {
                self.dlinks[u] = q;
                self.ulinks[d] = q;
                self.lengths[x] += 1;
                q -= 1;
            }
        }
    }
}

pub fn dlx<T>(sets: &Vec<Vec<T>>)
    where T: Eq + Hash {

}



