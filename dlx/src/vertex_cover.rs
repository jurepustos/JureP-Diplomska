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
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;
    use std::cmp::min;
    use libdlx::dlxc::*;
    use std::cmp::max;
    use libdlx::dlxc::Item;
    
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    enum Primary {
        Vertex(usize),
        SizeConstraint(usize)
    }
    
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    enum Secondary {
        Vertex(usize),
        SumVar(usize)
    }

    fn make_primaries(graph: &BTreeMap<usize, Vec<usize>>) -> Vec<Primary> {
        let mut primaries = Vec::new();
        for (a, neighbors) in graph {
            if neighbors.len() > 0 {
                primaries.push(Primary::Vertex(*a));
            }
        }

        for i in 0..primaries.len()-1 {
            primaries.push(Primary::SizeConstraint(i));
        }
    
        primaries
    }
    
    fn make_secondaries(graph: &BTreeMap<usize, Vec<usize>>) -> Vec<Secondary> {
        let mut secondaries = Vec::new();       
        for (i, neighbors) in graph {
            if neighbors.len() > 0 {
                secondaries.push(Secondary::Vertex(*i));
            }
        }
    
        for i in 0..graph.len()-1 {
            secondaries.push(Secondary::SumVar(i));
        }
    
        secondaries
    }
    
    fn add_edge_options(sets: &mut Vec<Vec<Item<Primary, Secondary, usize>>>, graph: &BTreeMap<usize, Vec<usize>>, ignored: &BTreeSet<usize>, presets: &BTreeSet<usize>) {
        let mut preset_set = vec![];
        for a in presets {
            preset_set.push(Item::Primary(Primary::Vertex(*a)));
        }
        for b in ignored {
            preset_set.push(Item::Primary(Primary::Vertex(*b)));
        }
        for a in presets {
            if *a == 760 {
                println!("1");
            }
            preset_set.push(Item::ColoredSecondary(Secondary::Vertex(*a), 1));
        }
        for b in ignored {
            if *b == 760 {
                println!("2");
            }
            preset_set.push(Item::ColoredSecondary(Secondary::Vertex(*b), 0));
        }
        sets.push(preset_set);

        for (a, neighbors) in graph {
            if neighbors.len() > 0 {
                if !ignored.contains(a) {
                    sets.push(vec![
                        Item::Primary(Primary::Vertex(*a)),
                        Item::ColoredSecondary(Secondary::Vertex(*a), 1)
                    ]);
                }
    
                if !presets.contains(a) {
                    let mut exclude_set = vec![
                        Item::Primary(Primary::Vertex(*a)),
                        Item::ColoredSecondary(Secondary::Vertex(*a), 0)
                    ];
                    for b in neighbors {
                        exclude_set.push(Item::ColoredSecondary(Secondary::Vertex(*b), 1));
                    }
                    sets.push(exclude_set);
                }
            }
        }
    }
    
    fn add_starting_sum_options(sets: &mut Vec<Vec<Item<Primary, Secondary, usize>>>, v0: usize, v1: usize) {
        sets.push(vec![
            Item::Primary(Primary::SizeConstraint(0)),
            Item::ColoredSecondary(Secondary::Vertex(v0), 0),
            Item::ColoredSecondary(Secondary::Vertex(v1), 1),
            Item::ColoredSecondary(Secondary::SumVar(0), 1)
        ]);

        sets.push(vec![
            Item::Primary(Primary::SizeConstraint(0)),
            Item::ColoredSecondary(Secondary::Vertex(v0), 1),
            Item::ColoredSecondary(Secondary::Vertex(v1), 0),
            Item::ColoredSecondary(Secondary::SumVar(0), 1)
        ]);
        
        sets.push(vec![
            Item::Primary(Primary::SizeConstraint(0)),
            Item::ColoredSecondary(Secondary::Vertex(v0), 0),
            Item::ColoredSecondary(Secondary::Vertex(v1), 0),
            Item::ColoredSecondary(Secondary::SumVar(0), 0)
        ]);
    
        sets.push(vec![
            Item::Primary(Primary::SizeConstraint(0)),
            Item::ColoredSecondary(Secondary::Vertex(v0), 1),
            Item::ColoredSecondary(Secondary::Vertex(v1), 1),
            Item::ColoredSecondary(Secondary::SumVar(0), 2)
        ]);
    }
    
    fn add_sum_options(sets: &mut Vec<Vec<Item<Primary, Secondary, usize>>>, graph: &BTreeMap<usize, Vec<usize>>, 
                       cover_size: usize, ignored: &BTreeSet<usize>, presets: &BTreeSet<usize>) {
        let vertices: Vec<usize> = graph.keys().cloned().collect();
        if vertices.len() < 2 {
            return
        }

        add_starting_sum_options(sets, vertices[0], vertices[1]);

        for i in 1..vertices.len()-1 {
            for s in 0..min(i+2, cover_size) {
                if !presets.contains(&vertices[i+1]) {
                    sets.push(vec![
                        Item::Primary(Primary::SizeConstraint(i)),
                        Item::ColoredSecondary(Secondary::SumVar(i-1), s),
                        Item::ColoredSecondary(Secondary::Vertex(vertices[i+1]), 0),
                        Item::ColoredSecondary(Secondary::SumVar(i), s)
                    ]);
                }
    
                if !ignored.contains(&vertices[i+1]) {
                    sets.push(vec![
                        Item::Primary(Primary::SizeConstraint(i)),
                        Item::ColoredSecondary(Secondary::SumVar(i-1), s),
                        Item::ColoredSecondary(Secondary::Vertex(vertices[i+1]), 1),
                        Item::ColoredSecondary(Secondary::SumVar(i), s+1)
                    ]);
                }
            }
    
            if i+2 >= cover_size && !presets.contains(&vertices[i+1]) {
                sets.push(vec![
                    Item::Primary(Primary::SizeConstraint(i)),
                    Item::ColoredSecondary(Secondary::SumVar(i-1), cover_size),
                    Item::ColoredSecondary(Secondary::Vertex(vertices[i+1]), 0),
                    Item::ColoredSecondary(Secondary::SumVar(i), cover_size)
                ]);
            }
        }
    }

    fn degree_one_reduction(graph: &BTreeMap<usize, Vec<usize>>)  ->  (BTreeSet<usize>, BTreeSet<usize>) {
        let mut redundant_vertices = BTreeSet::<usize>::new();
        let mut guaranteed_vertices = BTreeSet::<usize>::new();
        for (a, neighbors) in graph.iter() {
            if neighbors.len() == 1 {
                redundant_vertices.insert(*a);
                
                for b in neighbors {
                    if !redundant_vertices.contains(b) {
                        guaranteed_vertices.insert(*b);
                    }
                }
            }
        }

        (redundant_vertices, guaranteed_vertices)
    }
    
    pub fn vc_dlxc(graph: &BTreeMap<usize, Vec<usize>>, cover_size: usize) -> Option<Vec<usize>> {
        let primaries = make_primaries(graph);
        let secondaries = make_secondaries(graph);
        let sizes: Vec<usize> = (0..=graph.len()).into_iter().collect();

        let (redundant_vertices, guaranteed_vertices) = degree_one_reduction(graph);
        if guaranteed_vertices.len() > cover_size {
            return None
        }

        let mut sets = Vec::new();
        add_edge_options(&mut sets, graph, &redundant_vertices, &guaranteed_vertices);
        add_sum_options(&mut sets, &graph, cover_size, &redundant_vertices, &guaranteed_vertices);

        // println!("{:?}", sets);
        if let Some((_, colors)) = dlxc_first_mp(sets, primaries, secondaries, sizes, 11) {
            // println!("colors = {:?}", colors);
            let mut vertex_cover = BTreeSet::<usize>::new();
            for (item, color) in colors {
                if let Secondary::Vertex(i) = item {
                    if let Some(1) = color {
                        vertex_cover.insert(i);
                    }
                }
            }

            Some(vertex_cover
                .into_iter()
                .collect())
        }
        else {
            None
        }
    }
}
