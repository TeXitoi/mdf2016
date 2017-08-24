extern crate petgraph;
#[macro_use]
extern crate structopt_derive;
extern crate structopt;
extern crate rand;

use structopt::StructOpt;
use std::io::BufRead;
use rand::Rng;

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

fn solve() {
    let stdin = std::io::stdin();
    let mut ints = std::io::BufReader::new(stdin.lock()).lines().flat_map(|l| {
        l.unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
    });
    let n = ints.next().unwrap() as usize;
    let mut g = petgraph::graph::Graph::<u32, (), petgraph::Undirected>::with_capacity(n, n - 1);
    while let (Some(u), Some(v)) = (ints.next(), ints.next()) {
        g.extend_with_edges(&[(u, v)]);
    }

    let mut dfs = petgraph::visit::DfsPostOrder::new(&g, 0.into());
    while let Some(u) = dfs.next(&g) {
        g[u] = g.neighbors(u).map(|v| g[v]).sum::<u32>() + 1;
    }

    //use petgraph::dot::{Dot, Config};
    //println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

    let n = g.node_count() as u32;
    let n2 = n / 2;
    let mut u = 0.into();
    loop {
        let (nb, v) = g.neighbors(u).filter_map(|v| {
            if g[v] > g[u] {
                None
            } else {
                Some((g[v], v))
            }
        }).max().unwrap();
        if nb <= n2 { break; }
        u = v;
    }
    let (deconnected, v) = g.neighbors(u).map(|v| {
        if g[v] > g[u] {
            (n - g[u], v)
        } else {
            (g[v], v)
        }
    }).max().unwrap();
    println!("centroid: {}, nb_deconected = {}, critical_edge: {}-{}",
             u.index(), deconnected, u.index(), v.index());
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Gen { nb } => gen(nb),
        Opt::Solve => solve(),
    }
}
