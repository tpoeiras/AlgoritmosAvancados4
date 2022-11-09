use std::cell::RefCell;
use std::time::Instant;

use rand::prelude::*;
use rand::seq::index::sample;
use rand::seq::SliceRandom;

struct BipartiteGraph<T> {
    left_nodes: Vec<Node<T>>,
    right_nodes: Vec<Node<T>>,
}

impl<T> BipartiteGraph<T> {
    fn random(rng: &mut StdRng, l: usize, r: usize, num_edges: usize) -> BipartiteGraph<T>
    where
        T: Default,
    {
        let mut left_nodes = Vec::new();
        for _ in 0..l {
            left_nodes.push(Node {
                data: T::default(),
                neighbours: RefCell::new(Vec::new()),
            });
        }

        let mut right_nodes = Vec::new();
        for _ in 0..r {
            right_nodes.push(Node {
                data: T::default(),
                neighbours: RefCell::new(Vec::new()),
            });
        }

        let edges_codes = sample(rng, l * r, num_edges).into_vec();
        for code in edges_codes {
            let i = code / r;
            let j = code % r;
            left_nodes[i].neighbours.borrow_mut().push(j);
            right_nodes[j].neighbours.borrow_mut().push(i);
        }

        for i in 0..left_nodes.len() {
            left_nodes[i].neighbours.borrow_mut().sort();
        }

        for j in 0..right_nodes.len() {
            right_nodes[j].neighbours.borrow_mut().sort();
        }

        BipartiteGraph {
            left_nodes,
            right_nodes,
        }
    }

    fn kuhn<const R: bool>(&self, rng: &mut StdRng) -> Vec<Option<usize>> {
        let mut matched_right = vec![None; self.right_nodes.len()];
        for v in 0..self.left_nodes.len() {
            let mut used_left = vec![false; self.left_nodes.len()];
            self.try_kuhn::<R>(rng, v, &mut matched_right, &mut used_left);
        }

        matched_right
    }

    fn try_kuhn<const R: bool>(
        &self,
        rng: &mut StdRng,
        v: usize,
        matched_right: &mut Vec<Option<usize>>,
        used_left: &mut Vec<bool>,
    ) -> bool {
        if used_left[v] {
            return false;
        }

        used_left[v] = true;

        if R {
            let mut neighbours = self.left_nodes[v].neighbours.borrow_mut();
            (*neighbours).shuffle(rng);
        }

        let neighbours = self.left_nodes[v].neighbours.borrow();
        for &to in &*neighbours {
            if matched_right[to].is_none()
                || self.try_kuhn::<R>(rng, matched_right[to].unwrap(), matched_right, used_left)
            {
                matched_right[to] = Some(v);
                return true;
            }
        }

        false
    }

    fn print_as_dot(&self, matched_right: &[Option<usize>]) {
        println!("digraph A {{");
        println!("\trankdir=LR");
        println!("\tsplines=false");

        println!("\tsubgraph cluster1 {{");
        println!("\t\tmargin=30");
        println!("\t\tstyle=invis");
        for i in 0..self.left_nodes.len() {
            println!("\t\tA{i}");
        }
        println!("\t}}");
        println!("\tsubgraph cluster2 {{");
        println!("\t\tmargin=30");
        println!("\t\tstyle=invis");
        for i in 0..self.right_nodes.len() {
            println!("\t\tB{i}");
        }
        println!("\t}}");

        for i in 0..self.right_nodes.len() {
            if let Some(val) = matched_right[i] {
                let neighbours = self.right_nodes[i].neighbours.borrow();
                for &j in &*neighbours {
                    if j == val {
                        println!("\t\tB{i} -> A{j} [arrowhead=none,color=red]")
                    } else {
                        println!("\t\tB{i} -> A{j} [arrowhead=none]");
                    }
                }
            } else {
                let neighbours = self.right_nodes[i].neighbours.borrow();
                for j in &*neighbours {
                    println!("\t\tB{i} -> A{j} [arrowhead=none]");
                }
            }
        }

        println!("}}");
    }
}

struct Node<T> {
    data: T,
    neighbours: RefCell<Vec<usize>>,
}

fn test_graph<const R: bool>(rng: &mut StdRng, l: usize, r: usize, edges: usize) -> u128 {
    let graph = BipartiteGraph::<()>::random(rng, l, r, edges);

    let start = Instant::now();
    let _matched = graph.kuhn::<R>(rng);
    start.elapsed().as_nanos()
}

fn main() {
    let mut rng = StdRng::seed_from_u64(131254153212);

    let l = 10000;
    let r = 10000;
    println!("n,m,time");
    for n_edges in ((l * r / 200)..(l * r / 20)).step_by(l * r / 200) {
        for _ in 0..10 {
            let time = test_graph::<false>(&mut rng, l, r, n_edges);
            println!("{},{n_edges},{time}", l * r);
        }
    }
}
