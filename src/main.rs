extern crate petgraph;
#[macro_use]
extern crate structopt_derive;
extern crate structopt;
extern crate rand;

use structopt::StructOpt;
use std::io::BufRead;
use rand::Rng;

type Graph = petgraph::graph::Graph<u32, (), petgraph::Undirected>;
type NodeId = petgraph::graph::NodeIndex;

#[derive(StructOpt)]
enum Opt {
    #[structopt(name = "gen")]
    Gen {
        /// Number of vertices.
        nb: u32
    },
    #[structopt(name = "solve")]
    Solve,
}

fn gen(nb: u32) {
    println!("{}", nb);
    let mut rnd = rand::XorShiftRng::new_unseeded();
    for u in 1..nb {
        let v = rnd.gen_range::<u32>(0, u);
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
            .map(|v| if g[v] > g[u] { (n - g[u], v) } else { (g[v], v) })
            .max()
            .unwrap()
    }

    let n2 = g.node_count() as u32 / 2;
    let mut u = 0.into();
    loop {
        let (nb, v) = max_deconnected(u, &g);
        if nb <= n2 { break; }
        u = v;
    }
    let (deconnected, v) = max_deconnected(u, &g);
    println!("nb_deconected = {}, centroid: {}, critical_edge: {}-{}",
             u.index(), deconnected, u.index(), v.index());
}

fn main() {
    match Opt::from_args() {
        Opt::Gen { nb } => gen(nb),
        Opt::Solve => solve(),
    }
}
