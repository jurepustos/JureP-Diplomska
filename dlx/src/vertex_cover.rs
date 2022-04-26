use std::cmp::min;
use std::cmp::max;
use libdlx::dlxc::*;

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
    add_starting_sum_options(&mut sets);
    add_sum_options(&mut sets, max_vertex, cover_size);

    if let Some(result) = dlxc_first(sets, primaries, secondaries, sizes) {
        let mut vertex_cover = Vec::new();
        for set in result {
            for item in set {
                if let Item::ColoredSecondary(Secondary::Vertex(i), 1) = item {
                    vertex_cover.push(i);
                }
            }
        }
        Some(vertex_cover)
    }
    else {
        None
    }
}
