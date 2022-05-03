use std::collections::BTreeSet;

pub use dlx::*;

pub fn check_vertex_cover(graph_edges: &Vec<(usize, usize)>, cover: &BTreeSet<usize>) -> bool {
    let mut is_covered = true;
    for (a, b) in graph_edges {
        if !cover.contains(a) && !cover.contains(b) {
            is_covered = false;
        }
    }

    is_covered
}

mod dlx {
    use std::cmp::min;
    use libdlx::dlxc::*;
    use std::cmp::max;
    use libdlx::dlxc::Item;
    
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    enum Primary {
        Edge(usize, usize),
        SizeConstraint(usize)
    }
    
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    enum Secondary {
        Vertex(usize),
        SumVar(usize)
    }
    
    fn make_primaries(graph_edges: &Vec<(usize, usize)>, max_vertex: usize) -> Vec<Primary> {
        let mut primaries = Vec::new();
        for (a, b) in graph_edges {
            primaries.push(Primary::Edge(*a, *b));
        }
    
        for i in 0..max_vertex {
            primaries.push(Primary::SizeConstraint(i));
        }
    
        primaries
    }
    
    fn make_secondaries(max_vertex: usize) -> Vec<Secondary> {
        let mut secondaries = Vec::new();       
        for i in 0..=max_vertex {
            secondaries.push(Secondary::Vertex(i));
        }
    
        for i in 0..max_vertex {
            secondaries.push(Secondary::SumVar(i));
        }
    
        secondaries
    }
    
    fn add_edge_options(sets: &mut Vec<Vec<Item<Primary, Secondary, usize>>>, graph_edges: &Vec<(usize, usize)>) {
        for (a, b) in graph_edges {
            sets.push(vec![
                Item::Primary(Primary::Edge(*a, *b)),
                Item::ColoredSecondary(Secondary::Vertex(*a), 0),
                Item::ColoredSecondary(Secondary::Vertex(*b), 1)
            ]);
    
            sets.push(vec![
                Item::Primary(Primary::Edge(*a, *b)),
                Item::ColoredSecondary(Secondary::Vertex(*a), 1),
                Item::ColoredSecondary(Secondary::Vertex(*b), 0)
            ]);
    
            sets.push(vec![
                Item::Primary(Primary::Edge(*a, *b)),
                Item::ColoredSecondary(Secondary::Vertex(*a), 1),
                Item::ColoredSecondary(Secondary::Vertex(*b), 1)
            ]);
        }
    }
    
    fn add_starting_sum_options(sets: &mut Vec<Vec<Item<Primary, Secondary, usize>>>) {
        sets.push(vec![
            Item::Primary(Primary::SizeConstraint(0)),
            Item::ColoredSecondary(Secondary::Vertex(0), 0),
            Item::ColoredSecondary(Secondary::Vertex(1), 0),
            Item::ColoredSecondary(Secondary::SumVar(0), 0)
        ]);
    
        sets.push(vec![
            Item::Primary(Primary::SizeConstraint(0)),
            Item::ColoredSecondary(Secondary::Vertex(0), 0),
            Item::ColoredSecondary(Secondary::Vertex(1), 1),
            Item::ColoredSecondary(Secondary::SumVar(0), 1)
        ]);
    
        sets.push(vec![
            Item::Primary(Primary::SizeConstraint(0)),
            Item::ColoredSecondary(Secondary::Vertex(0), 1),
            Item::ColoredSecondary(Secondary::Vertex(1), 0),
            Item::ColoredSecondary(Secondary::SumVar(0), 1)
        ]);
    
        sets.push(vec![
            Item::Primary(Primary::SizeConstraint(0)),
            Item::ColoredSecondary(Secondary::Vertex(0), 1),
            Item::ColoredSecondary(Secondary::Vertex(1), 1),
            Item::ColoredSecondary(Secondary::SumVar(0), 2)
        ]);
    }
    
    fn add_sum_options(sets: &mut Vec<Vec<Item<Primary, Secondary, usize>>>, max_vertex: usize, cover_size: usize) {
        add_starting_sum_options(sets);

        for i in 1..max_vertex {
            for s in 0..min(i+2, cover_size) {
                sets.push(vec![
                    Item::Primary(Primary::SizeConstraint(i)),
                    Item::ColoredSecondary(Secondary::SumVar(i-1), s),
                    Item::ColoredSecondary(Secondary::Vertex(i+1), 0),
                    Item::ColoredSecondary(Secondary::SumVar(i), s)
                ]);
    
                sets.push(vec![
                    Item::Primary(Primary::SizeConstraint(i)),
                    Item::ColoredSecondary(Secondary::SumVar(i-1), s),
                    Item::ColoredSecondary(Secondary::Vertex(i+1), 1),
                    Item::ColoredSecondary(Secondary::SumVar(i), s+1)
                ]);
            }
    
            if i+2 >= cover_size {
                sets.push(vec![
                    Item::Primary(Primary::SizeConstraint(i)),
                    Item::ColoredSecondary(Secondary::SumVar(i-1), cover_size),
                    Item::ColoredSecondary(Secondary::Vertex(i+1), 0),
                    Item::ColoredSecondary(Secondary::SumVar(i), cover_size)
                ]);
            }
        }
    }
    
    pub fn vc_dlxc(graph_edges: &Vec<(usize, usize)>, cover_size: usize) -> Option<Vec<usize>> {
        let max_vertex = graph_edges
            .iter()
            .map(|(a, b)| max(*a, *b))
            .max()
            .unwrap_or(2);
    
        let primaries = make_primaries(graph_edges, max_vertex);
        let secondaries = make_secondaries(max_vertex);
        let sizes: Vec<usize> = (0..=max_vertex+1).into_iter().collect();
    
        let mut sets = Vec::new();
        add_edge_options(&mut sets, graph_edges);
        add_sum_options(&mut sets, max_vertex, cover_size);
    
        if let Some((_, colors)) = dlxc_first(sets, primaries, secondaries, sizes) {
            let mut vertex_cover = vec![false; max_vertex+1];
            for (item, color) in colors {
                if let Secondary::Vertex(i) = item {
                    if let Some(1) = color {
                        vertex_cover[i] = true;
                    }
                }
            }

            Some(vertex_cover
                .into_iter()
                .enumerate()
                .filter(|(_, selected)| *selected)
                .map(|(i, _)| i)
                .collect())
        }
        else {
            None
        }
    }
}
