use petgraph::prelude::Graph;
use petgraph::dot::{Dot, Config};

use pmi3sat::cnf::*;
use pmi3sat::sat::SAT;

fn main() {
    let mut graph = Graph::<&str, (), petgraph::Undirected>::new_undirected();
    let t = graph.add_node("T");
    let f = graph.add_node("F");
    let b = graph.add_node("B");
    graph.add_edge(t, f, ());
    graph.add_edge(f, b, ());
    graph.add_edge(t, b, ());

    let n = 5;

    let mut clauses = vec![];
    for _ in 0..n {
        clauses.push(Clause::random(n));
    }
    let sat: SAT = SAT::new(clauses);


    for clause in sat.clauses {
        let lits = clause.literals;
        let lits_str: Vec<(String, String)> = lits.iter().map(|l| (l.to_string(), { let mut s = String::from("~"); s.push_str(&l.to_string()); s })).collect();
        for (l, lp) in std::iter::zip(&lits, &lits_str) {
            let il = graph.add_node(&lp.0);
            let ilp = graph.add_node(&lp.1);
            let or1 = graph.add_node("-");
            let or2 = graph.add_node("-");
            let or3 = graph.add_node("-");
            graph.add_edge(il, ilp, ());
            graph.add_edge(il, b, ());
            graph.add_edge(ilp, b, ());
            match l.negated {
                Negated::YES => graph.add_edge(ilp, or1, ()),
                Negated::NO => graph.add_edge(il, or1, ()),
            };
            graph.add_edge(or1, or2, ());
            graph.add_edge(or1, or3, ());
            graph.add_edge(or2, or3, ());
        }
    }
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
}
