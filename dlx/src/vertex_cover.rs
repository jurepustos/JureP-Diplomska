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
    use std::time::Instant;
use std::time::Duration;
use libdlx::min_cost_dlxc::min_cost_dlxc_iter;
    use std::collections::VecDeque;
    use crate::dlxc::dlxc_iter;
    use crate::min_cost_dlxc::min_cost_dlxc_first;
    use crate::dlxc::dlxc_first;
    // use libdlx::dlxc::Item;
    use libdlx::min_cost_dlxc::Item;
    use libdlx::min_cost_dlxc::min_cost_dlxc;
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;
    use std::cmp::min;
    use std::cmp::max;
    
    type Graph = BTreeMap<usize, BTreeSet<usize>>;

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

    #[derive(Clone,Copy,PartialEq,Eq,Debug)]
    struct DegreeTwoFold {
        vertex: usize,
        neighbors: [usize; 2],
        new_vertex: usize
    }

    #[derive(Clone,Copy,PartialEq,Eq,Debug)]
    struct TwinFold {
        twins: [usize; 2],
        neighbors: [usize; 3],
        new_vertex: usize
    }

    #[derive(Clone,PartialEq,Eq,Debug)]
    struct Reductions {
        exclusions: BTreeSet<usize>,
        inclusions: BTreeSet<usize>,
        degree_two_folds: Vec<DegreeTwoFold>,
        twin_folds: Vec<TwinFold>,
        next_vertex: usize
    }

    impl Reductions {
        fn new(max_vertex: usize) -> Self {
            Reductions {
                exclusions: BTreeSet::new(),
                inclusions: BTreeSet::new(),
                degree_two_folds: Vec::new(),
                twin_folds: Vec::new(),
                next_vertex: max_vertex + 1
            }
        }

        fn len(&self) -> usize {
            self.exclusions.len() + self.inclusions.len() + self.degree_two_folds.len() + self.twin_folds.len()
        }
    }

    fn make_primaries(graph: &Graph) -> Vec<Primary> {
        let mut primaries = Vec::new();
        for (a, neighbors) in graph {
            if neighbors.len() > 0 {
                primaries.push(Primary::Vertex(*a));
            }
        }
    
        primaries
    }
    
    fn make_secondaries(graph: &Graph) -> Vec<Secondary> {
        let mut secondaries = Vec::new();       
        for (i, neighbors) in graph {
            if neighbors.len() > 0 {
                secondaries.push(Secondary::Vertex(*i));
            }
        }
    
        secondaries
    }
    
    fn add_edge_options(sets: &mut Vec<(Vec<Item<Primary, Secondary, usize>>, usize)>, graph: &Graph) {
        for (a, neighbors) in graph {
            if neighbors.len() > 0 {
                sets.push((vec![
                    Item::Primary(Primary::Vertex(*a)),
                    Item::ColoredSecondary(Secondary::Vertex(*a), 1)
                ], 1));
    
                let mut exclude_set = vec![
                    Item::Primary(Primary::Vertex(*a)),
                    Item::ColoredSecondary(Secondary::Vertex(*a), 0)
                ];
                for b in neighbors {
                    exclude_set.push(Item::ColoredSecondary(Secondary::Vertex(*b), 1));
                }
                sets.push((exclude_set, 0));  
            }
        }
    }

    fn delete_vertex(graph: &mut Graph, vertex: usize) {
        if let Some(neighbors) = graph.remove(&vertex) {
            for n in neighbors {
                let n_neighbors = graph.get_mut(&n).unwrap();
                n_neighbors.remove(&vertex);
                
                if n_neighbors.is_empty() {
                    graph.remove(&n);
                }
            }
        }
    }

    fn degree_one_reduction(graph: &mut Graph, reductions: &mut Reductions) {
        let vertices: Vec<usize> = graph.keys().cloned().collect();
        for a in vertices {
            if graph.contains_key(&a) && graph[&a].len() == 1 {
                let b = graph[&a].iter().next().cloned().unwrap();
                reductions.exclusions.insert(a);
                reductions.inclusions.insert(b);
                delete_vertex(graph, a);
                delete_vertex(graph, b);
            }
        }
    }

    fn dominance_reduction(graph: &mut Graph, reductions: &mut Reductions) {
        let vertices: Vec<usize> = graph.keys().cloned().collect();
        for &a in &vertices {
            for &b in vertices.iter().filter(|b| **b > a) {
                let mut a_neighbors = graph[&a].clone();
                a_neighbors.insert(a);

                let mut b_neighbors = graph[&b].clone();
                b_neighbors.insert(b);

                if a_neighbors.is_superset(&b_neighbors) {
                    reductions.inclusions.insert(a);
                    delete_vertex(graph, a);
                }
            }
        }
    }

    fn merge_vertices(graph: &mut Graph, vertices: &[usize], new_vertex: usize) {
        let mut new_neighbors = BTreeSet::new();
        for &v in vertices {
            new_neighbors.append(&mut graph[&v].clone());
        }

        for &v in vertices {
            new_neighbors.remove(&v);
        }

        for &n in &new_neighbors {
            let new_neighbors = graph.get_mut(&n).unwrap();
            new_neighbors.insert(new_vertex);
        }

        for &v in vertices {
            delete_vertex(graph, v);
        }
        
        if !new_neighbors.is_empty() {
            graph.insert(new_vertex, new_neighbors);
        }
    }

    fn degree_two_reduction(graph: &mut Graph, reductions: &mut Reductions) {
        let vertices: Vec<usize> = graph.keys().cloned().collect();
        for a in vertices {
            if graph.contains_key(&a) && graph[&a].len() == 2 {
                let neighbor_list: Vec<usize> = graph[&a].iter().cloned().collect();
                let v1 = neighbor_list[0];
                let v2 = neighbor_list[1];
                if !graph[&v1].contains(&v2) {
                    reductions.degree_two_folds.push(DegreeTwoFold {
                        vertex: a,
                        neighbors: [v1, v2],
                        new_vertex: reductions.next_vertex
                    });

                    merge_vertices(graph, &[a, v1, v2], reductions.next_vertex);
                    reductions.next_vertex += 1;
                }
                else {
                    reductions.exclusions.insert(a);
                    reductions.inclusions.insert(v1);
                    reductions.inclusions.insert(v2);
                    delete_vertex(graph, a);
                    delete_vertex(graph, v1);
                    delete_vertex(graph, v2);
                }
            }
        }
    }

    fn twin_reduction(graph: &mut Graph, reductions: &mut Reductions) {
        let vertices: Vec<usize> = graph.keys().cloned().collect();
        for a in vertices {
            if graph.contains_key(&a) && graph[&a].len() == 3 {
                let neighbor_list: Vec<usize> = graph[&a].iter().cloned().collect();
                let v1 = neighbor_list[0];
                let v2 = neighbor_list[1];
                let v3 = neighbor_list[2];
                let intersection = graph[&v1]
                    .intersection(&graph[&v2])
                    .cloned()
                    .collect::<BTreeSet<usize>>()
                    .intersection(&graph[&v3])
                    .cloned()
                    .collect::<BTreeSet<usize>>();
                
                if let Some(b) = intersection.into_iter().next() {
                    if graph[&v1].contains(&v2) || graph[&v1].contains(&v3) || graph[&v2].contains(&v3) {
                        reductions.exclusions.insert(a);
                        reductions.exclusions.insert(b);
                        reductions.inclusions.insert(v1);
                        reductions.inclusions.insert(v2);
                        reductions.inclusions.insert(v3);
                        delete_vertex(graph, a);
                        delete_vertex(graph, b);
                        delete_vertex(graph, v1);
                        delete_vertex(graph, v2);
                        delete_vertex(graph, v3);
                    }
                    else {
                        reductions.twin_folds.push(TwinFold {
                            twins: [a, b],
                            neighbors: [v1, v2, v3],
                            new_vertex: reductions.next_vertex
                        });

                        let mut new_neighbors = graph[&v1].clone();
                        new_neighbors.append(&mut graph[&v2].clone());
                        new_neighbors.append(&mut graph[&v3].clone());

                        merge_vertices(graph, &[a, b, v1, v2, v3], reductions.next_vertex);
                        reductions.next_vertex += 1;
                    }
                }

            }
        }
    }

    fn unconfined_vertex(graph: &Graph, vertex: usize) -> bool {
        let mut group = BTreeSet::new();
        group.insert(vertex);

        // the algorithm is guaranteed to return, in the absolute worst case when s has all vertices
        loop {
            let mut neighborhood = BTreeSet::<usize>::new();
            for b in &group {
                neighborhood.append(&mut graph[b].clone());
            }
    
            let mut intersections = Vec::<BTreeSet<usize>>::new();
            for b in &neighborhood {
                let intersection: BTreeSet<usize> = graph.get(b)
                    .unwrap_or(&BTreeSet::new())
                    .intersection(&group)
                    .into_iter()
                    .cloned()
                    .collect();
                
                if intersection.len() == 1 {
                    let diff = graph[b]
                        .difference(&neighborhood)
                        .into_iter()
                        .cloned()
                        .collect::<BTreeSet<usize>>()
                        .difference(&group)
                        .into_iter()
                        .cloned()
                        .collect();
                    intersections.push(diff);
                }
            }
    
            if intersections.is_empty() {
                return false
            }
    
            let min_intersection = intersections
                .into_iter()
                .min_by(|set1, set2| set1.len().cmp(&set2.len()))
                .unwrap_or(BTreeSet::new());
    
            if min_intersection.is_empty() {
                return true
            }
            else if min_intersection.len() == 1 {
                let vertex = min_intersection.into_iter().next().unwrap();
                group.insert(vertex);
            }
            else {
                return false
            }
        }
    }
    
    fn unconfined_reduction(graph: &mut Graph, reductions: &mut Reductions) {
        let vertices: Vec<usize> = graph.keys().cloned().collect();
        for a in vertices {
            if graph.contains_key(&a) && unconfined_vertex(graph, a) {
                reductions.inclusions.insert(a);
                delete_vertex(graph, a);
            }
        }
    }

    fn reduction_round(graph: &mut Graph, reductions: &mut Reductions) {
        degree_one_reduction(graph, reductions);
        degree_two_reduction(graph, reductions);
        twin_reduction(graph, reductions);
        dominance_reduction(graph, reductions);
        unconfined_reduction(graph, reductions);
    }

    fn reduce_graph(graph: &mut Graph) -> Reductions {
        let max_vertex = graph.keys().cloned().max().unwrap_or(0);
        let mut reductions = Reductions::new(max_vertex);

        let mut reductions_count = reductions.len();
        reduction_round(graph, &mut reductions);
        while reductions.len() > reductions_count {
            reductions_count = reductions.len();
            reduction_round(graph, &mut reductions);
        }

        reductions
    }

    fn get_unvisited_vertex(graph: &Graph, visited_vertices: &Vec<bool>) -> Option<usize> {
        graph.keys()
            .filter(|v| !visited_vertices[**v])
            .next()
            .cloned()
    }

    fn get_connected_components(graph: &Graph) -> Vec<Graph> {
        let max_vertex = graph.keys().max().cloned().unwrap_or(0);
        let mut visited_vertices = vec![false; max_vertex+1];
        let mut components = Vec::new();

        while let Some(first_vertex) = get_unvisited_vertex(graph, &visited_vertices) {
            let mut queue = VecDeque::new();
            let mut component = BTreeMap::new();

            visited_vertices[first_vertex] = true;
            component.insert(first_vertex, graph[&first_vertex].clone());
            queue.push_front(first_vertex);
            
            while let Some(v) = queue.pop_back() {
                for u in &graph[&v] {
                    if !visited_vertices[*u] {
                        visited_vertices[*u] = true;
                        component.insert(*u, graph[u].clone());
                        queue.push_front(*u);
                    }
                }
            }
    
            components.push(component);
        }

        components
    }

    fn unreduce_cover(cover: &mut BTreeSet<usize>, reductions: &Reductions) {
        cover.append(&mut reductions.inclusions.clone());

        for fold in reductions.twin_folds.iter().rev() {
            if cover.contains(&fold.new_vertex) {
                cover.remove(&fold.new_vertex);
                for n in fold.neighbors {
                    cover.insert(n);
                }
            }
            else {
                cover.insert(fold.twins[0]);
                cover.insert(fold.twins[1]);
            }
        }

        for fold in reductions.degree_two_folds.iter().rev() {
            if cover.contains(&fold.new_vertex) {
                cover.remove(&fold.new_vertex);
                for n in fold.neighbors {
                    cover.insert(n);
                }
            }
            else {
                cover.insert(fold.vertex);
            }
        }
    }

    fn component_cover(graph: &Graph, time_limit: Duration) -> Option<Vec<usize>> {
        if graph.is_empty() {
            return Some(Vec::new());
        }

        let primaries = make_primaries(&graph);
        let secondaries = make_secondaries(&graph);
        let sizes: Vec<usize> = (0..=graph.len()).into_iter().collect();
        
        let mut sets = Vec::new();
        add_edge_options(&mut sets, &graph);

        let mut iter = min_cost_dlxc_iter(sets, primaries, secondaries, sizes);
        let mut cover = BTreeSet::<usize>::new();
        let start_time = Instant::now();
        while let Some(solution) = iter.next() {
            cover = BTreeSet::new();
            for (item, color) in solution.colors {
                if let Secondary::Vertex(i) = item {
                    if let Some(1) = color {
                        cover.insert(i);
                    }
                }
            }

            if start_time.elapsed() > time_limit {
                return None
            }
        }
        
        Some(cover.into_iter().collect())
    }

    pub fn vc_dlxc(mut graph: Graph, time_limit: Duration) -> Option<Vec<usize>> {
        let start_time = Instant::now();
        let mut full_cover = BTreeSet::<usize>::new();
        let reductions = reduce_graph(&mut graph);
        let components = get_connected_components(&graph);
        for component in components {
            if start_time.elapsed() >= time_limit {
                return Some(Vec::new())
            }
            if let Some(cover) = component_cover(&component, time_limit - start_time.elapsed()) {
                for v in cover {
                    full_cover.insert(v);
                }
            }
            else {
                return None
            }
        }
        unreduce_cover(&mut full_cover, &reductions);
        Some(full_cover.into_iter().collect())
    }
}
