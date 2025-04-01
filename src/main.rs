use clap::Parser;
use rand::Rng;
use rand::SeedableRng;
use std::io::BufRead;

type Graph = petgraph::graph::Graph<u32, (), petgraph::Undirected>;
type NodeId = petgraph::graph::NodeIndex;

#[derive(Parser)]
enum Opt {
    Gen {
        /// Number of vertices.
        nb: u32,
    },
    Solve,
}

fn generate(nb: u32) {
    println!("{}", nb);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(42);
    for u in 1..nb {
        let v = rnd.random_range(0..u);
        println!("{} {}", u, v);
    }
}

fn read_graph() -> Graph {
    let stdin = std::io::stdin();
    let mut ints = std::io::BufReader::new(stdin.lock()).lines().flat_map(|l| {
        l.unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
    });
    let n = ints.next().unwrap() as usize;
    let mut g = Graph::with_capacity(n, n - 1);
    while let (Some(u), Some(v)) = (ints.next(), ints.next()) {
        g.extend_with_edges(&[(u, v)]);
    }
    g
}

fn solve() {
    let mut g = read_graph();

    let mut dfs = petgraph::visit::DfsPostOrder::new(&g, 0.into());
    while let Some(u) = dfs.next(&g) {
        g[u] = g.neighbors(u).map(|v| g[v]).sum::<u32>() + 1;
    }

    //use petgraph::dot::{Dot, Config};
    //println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

    fn max_deconnected(u: NodeId, g: &Graph) -> (u32, NodeId) {
        let n = g.node_count() as u32;
        g.neighbors(u)
            .map(|v| {
                if g[v] > g[u] {
                    (n - g[u], v)
                } else {
                    (g[v], v)
                }
            })
            .max()
            .unwrap()
    }

    let n2 = g.node_count() as u32 / 2;
    let mut u = 0.into();
    loop {
        let (nb, v) = max_deconnected(u, &g);
        if nb <= n2 {
            break;
        }
        u = v;
    }
    let (deconnected, v) = max_deconnected(u, &g);
    println!(
        "nb_deconected = {}, centroid: {}, critical_edge: {}-{}",
        deconnected,
        u.index(),
        u.index(),
        v.index()
    );
}

fn main() {
    match Opt::parse() {
        Opt::Gen { nb } => generate(nb),
        Opt::Solve => solve(),
    }
}
