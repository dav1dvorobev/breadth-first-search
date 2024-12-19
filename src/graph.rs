use rand::{seq::SliceRandom, Rng};

#[derive(Debug)]
pub struct Graph {
    adjacency_list: Vec<Vec<usize>>,
    vertecs_quantity: usize,
    edges_quantity: usize,
}

impl Graph {
    pub fn rand(v: usize, e: usize) -> Self {
        if e < v - 1 || e > v * (v - 1) / 2 {
            panic!("wrong edges quantity");
        }
        let mut adjacency_list = vec![vec![]; v];
        let mut rng = rand::thread_rng();
        for i in 1..v {
            let parent = rng.gen_range(0..i);
            adjacency_list[i].push(parent);
            adjacency_list[parent].push(i);
        }
        let mut all_edges: Vec<(usize, usize)> = Vec::new();
        for i in 0..v {
            for j in (i + 1)..v {
                all_edges.push((i, j));
            }
        }
        all_edges.retain(|&(a, b)| !adjacency_list[a].contains(&b));
        all_edges.shuffle(&mut rng);
        for &(a, b) in all_edges.iter().take(e - (v - 1)) {
            adjacency_list[a].push(b);
            adjacency_list[b].push(a);
        }
        Self {
            adjacency_list: adjacency_list,
            vertecs_quantity: v,
            edges_quantity: e,
        }
    }
    pub fn adjacency_list(&self) -> &Vec<Vec<usize>> {
        &self.adjacency_list
    }
    pub fn vertecs(&self) -> usize {
        self.vertecs_quantity
    }
    pub fn edges(&self) -> usize {
        self.edges_quantity
    }
}
