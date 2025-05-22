fn bfs(graph: &Vec<Vec<usize>>, start: usize) {
    let mut visited = vec![false; graph.len()];
    let mut queue = vec![start];
    visited[start] = true;

    let mut front = 0;

    while front < queue.len() {
        let node = queue[front];
        front += 1;

        println!("Visited {}", node);

        for &neighbor in &graph[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push(neighbor);
            }
        }
    }
}

fn main() {
    let graph = vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]];
    println!("graph: {:?}", graph);

    println!("BFS Traversal:");
    bfs(&graph, 0);
}
