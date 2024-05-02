pub mod cc {
    type Component = usize;
    fn mark_component_bfs(vertex:usize, adjacency_list: &Vec<Vec<usize>>, component:&mut Vec<Option<Component>>, component_no:Component) {
        component[vertex] = Some(component_no);
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(vertex);
        while let Some(v) = queue.pop_front() {
            for w in adjacency_list[v].iter() {
                if let None = component[*w] {
                    component[*w] = Some(component_no);
                    queue.push_back(*w);
                }
            }
        }
    }
    
    pub fn verify_connected_components(n: usize, adjacency_list: Vec<Vec<usize>>) {
        let mut component: Vec<Option<Component>> = vec![None;n];
        let mut component_count = 0;
        for v in 0..n {
            if let None = component[v] {
                component_count += 1;
                mark_component_bfs(v, &adjacency_list, &mut component, component_count);
            }
        };
        println!("There are {:?} connected components", component_count);
    }
}