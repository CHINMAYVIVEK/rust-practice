fn dfs(graph: &Vec<Vec<usize>>, node: usize, visited: &mut [bool]) {
    if visited[node] {
        return;
    }

    visited[node] = true;
    println!("Visited {}", node);

    for &neighbor in &graph[node] {
        dfs(graph, neighbor, visited);
    }
}

fn main() {
    let graph = vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]];
    println!("graph: {:?}", graph);

    println!("DFS Traversal:");
    let mut visited_dfs = vec![false; graph.len()];
    dfs(&graph, 0, &mut visited_dfs);
}
