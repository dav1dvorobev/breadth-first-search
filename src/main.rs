mod graph;

use anyhow::Result;
use graph::Graph;
use std::{collections::VecDeque, fs::OpenOptions, io::Write, time::Instant};

fn main() -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("output.csv")?;
    file.write_all(b"v,best,medium,worst\n")?;
    let mut buf = [0; 3];
    for v in 10..1_000 + 1 {
        let edges_quantities = [v, v * (v - 1) / 4, v * (v - 1) / 2];
        for i in 0..edges_quantities.len() {
            let graph = Graph::rand(v, edges_quantities[i]);
            let start_timestamp = Instant::now();
            let _ = bfs(&graph, 0);
            buf[i] = (Instant::now() - start_timestamp).as_nanos();
        }
        file.write_all(format!("{},{},{},{}\n", v, buf[0], buf[1], buf[2]).as_bytes())?;
    }
    Ok(())
}

fn bfs(graph: &Graph, start: usize) -> Vec<Option<usize>> {
    let vertecs = graph.vertecs();
    if start >= vertecs {
        panic!("out of vertecs");
    }
    let mut distances: Vec<Option<usize>> = vec![None; vertecs];
    let mut q: VecDeque<usize> = VecDeque::with_capacity(vertecs);
    distances[start] = Some(0);
    q.push_back(start);
    while let Some(v) = q.pop_front() {
        for &to in &graph.adjacency_list()[v] {
            if distances[to].is_none() {
                distances[to] = Some(distances[v].unwrap() + 1);
                q.push_back(to);
            }
        }
    }
    distances
}
