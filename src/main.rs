mod graph;

use anyhow::Result;
use graph::Graph;
use std::collections::VecDeque;
use tokio::{fs::OpenOptions, io::AsyncWriteExt, time::Instant};

#[tokio::main]
async fn main() -> Result<()> {
    tokio::fs::create_dir_all("output/").await?;
    let mut tasks = vec![];
    for i in 1..=10 {
        let task = tokio::spawn(async move {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(format!("output/data-{i}.csv"))
                .await
                .expect("failed to open");
            file.write_all(b"v,best,medium,worst\n")
                .await
                .expect("failed to write");
            let mut buf = [0; 3];
            for v in 10..=1000 {
                let edges_quantities = [v, v * (v - 1) / 4, v * (v - 1) / 2];
                for i in 0..edges_quantities.len() {
                    let graph = Graph::rand(v, edges_quantities[i]);
                    let start_timestamp = Instant::now();
                    let _ = bfs(&graph, 0);
                    buf[i] = (Instant::now() - start_timestamp).as_nanos();
                }
                file.write_all(format!("{},{},{},{}\n", v, buf[0], buf[1], buf[2]).as_bytes())
                    .await
                    .expect("failed to write");
            }
        });
        tasks.push(task);
    }
    for task in tasks { task.await?; }
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
