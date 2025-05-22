const INF: u32 = u32::MAX;

fn dijkstra(graph: &Vec<Vec<(usize, u32)>>, start: usize) -> Vec<u32> {
    let n = graph.len();
    let mut dist = vec![INF; n];
    let mut visited = vec![false; n];

    dist[start] = 0;

    for _ in 0..n {
        // Find the unvisited node with the smallest distance
        let mut u = n;
        for i in 0..n {
            if !visited[i] && (u == n || dist[i] < dist[u]) {
                u = i;
            }
        }

        if u == n || dist[u] == INF {
            break; // All reachable nodes have been visited
        }

        visited[u] = true;

        // Update distances for neighbors
        for &(v, weight) in &graph[u] {
            if dist[u] != INF && dist[u] + weight < dist[v] {
                dist[v] = dist[u] + weight;
            }
        }
    }

    dist
}

fn main() {
    let graph = vec![
        vec![(1, 2), (2, 4)],
        vec![(2, 1), (3, 7)],
        vec![(3, 3)],
        vec![],
    ];
    println!("graph: {:?}", graph);

    let dist = dijkstra(&graph, 0);
    println!("Dijkstra's Algorithm: {:?}", dist);
}
