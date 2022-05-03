use petgraph::prelude::Graph;
use petgraph::dot::{Dot, Config};

use pmi3sat::cnf::*;
use pmi3sat::sat::SAT;

fn main() {
    let mut graph = Graph::<String, (), petgraph::Undirected>::new_undirected();
    let t = graph.add_node(String::from("T"));
    let f = graph.add_node(String::from("F"));
    let b = graph.add_node(String::from("B"));
    graph.add_edge(t, f, ());
    graph.add_edge(f, b, ());
    graph.add_edge(t, b, ());

    let mut clauses = vec![];

    for _ in 0..3 {
        clauses.push(Clause::random(6));
    }
    let sat: SAT = SAT::new(clauses);
    println!("{}", &sat);

    let mut graph_str_: Vec<String> = vec![];

    let mut k = 1;

    graph_str_.push(format!("    subgraph cluster_0 {{").to_string());
    graph_str_.push(format!("        fontsize = 24;").to_string());
    graph_str_.push(format!("        label = \"*T-B-F*\";").to_string());
    graph_str_.push(format!("        0 -- 1 [ ]").to_string());
    graph_str_.push(format!("        0 -- 2 [ ]").to_string());
    graph_str_.push(format!("        2 -- 1 [ ]").to_string());
    graph_str_.push("    }".to_string());

    for clause in sat.clauses {
        graph_str_.push(format!("\n    subgraph cluster_{k} {{").to_string());
        graph_str_.push(format!("        fontsize = 24;").to_string());
        graph_str_.push(format!("        label = \"*{}*\";", clause.to_string()).to_string());
        let mut clause_edges = vec![];
        let org1_node = graph.add_node(String::from("-"));
        let org2_node = graph.add_node(String::from("-"));
        let org3_node = graph.add_node(String::from("-"));
        let org4_node = graph.add_node(String::from("-"));
        let org5_node = graph.add_node(String::from("-"));
        let org6_node = graph.add_node(String::from("-"));
        clause_edges.push(graph.update_edge(org1_node, org2_node, ()));
        clause_edges.push(graph.update_edge(org1_node, org3_node, ()));
        clause_edges.push(graph.update_edge(org2_node, org3_node, ()));
        clause_edges.push(graph.update_edge(org3_node, org4_node, ()));
        clause_edges.push(graph.update_edge(org5_node, org4_node, ()));
        clause_edges.push(graph.update_edge(org4_node, org6_node, ()));
        clause_edges.push(graph.update_edge(org5_node, org6_node, ()));
        clause_edges.push(graph.update_edge(org6_node, b, ()));
        clause_edges.push(graph.update_edge(org6_node, f, ()));
        let literals: Vec<String> = clause.literals.iter().map(|l| l.to_string()).collect();
        let literals_negated: Vec<String> = literals.iter().map(|l| { let mut s = String::from("¬"); s.push_str(&l.to_string()); s }).collect();
        let mut i = 0;
        for (lit_str, lit_neg_str) in std::iter::zip(literals, literals_negated) {
            let lit_node = graph.add_node(lit_str.clone());
            let lit_neg_node = graph.add_node(lit_neg_str.clone());
            clause_edges.push(graph.update_edge(lit_node, lit_neg_node, ()));
            clause_edges.push(graph.update_edge(lit_node, b, ()));
            clause_edges.push(graph.update_edge(lit_neg_node, b, ()));
            match i {
                0 => {
                    match clause.literals[0].negated {
                        Negated::YES => {
                            clause_edges.push(graph.update_edge(lit_neg_node, org1_node, ()));
                        },
                        Negated::NO => {
                            clause_edges.push(graph.update_edge(lit_node, org1_node, ()));
                        }
                    }
                },
                1 => {
                    match clause.literals[1].negated {
                        Negated::YES => {
                            clause_edges.push(graph.update_edge(lit_neg_node, org2_node, ()));
                        },
                        Negated::NO => {
                            clause_edges.push(graph.update_edge(lit_node, org2_node, ()));
                        }
                    }
                },
                2 => {
                    match clause.literals[2].negated {
                        Negated::YES => {
                            clause_edges.push(graph.update_edge(lit_neg_node, org5_node, ()));
                        },
                        Negated::NO => {
                            clause_edges.push(graph.update_edge(lit_node, org5_node, ()));
                        }
                    }
                },
                _ => ()
            };
            i += 1;
        }
        for edge in clause_edges {
            println!("\t{} -- {}", graph.edge_endpoints(edge).unwrap().0.index(), graph.edge_endpoints(edge).unwrap().1.index());
            graph_str_.push(format!("        {} -- {} []", graph.edge_endpoints(edge).unwrap().0.index(), graph.edge_endpoints(edge).unwrap().1.index()).to_string());
        }
        graph_str_.push("    }".to_string());
        k += 1;
    }

    let mut graph_str = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    graph_str = graph_str.replace("\\\"", "");
    graph_str = graph_str.replace("label = \"T\"", "label = \"T\" color = \"green\" style = \"filled\"");
    graph_str = graph_str.replace("label = \"F\"", "label = \"F\" color = \"red\" style = \"filled\"");
    graph_str = graph_str.replace("label = \"B\"", "label = \"B\" color = \"blue\" style = \"filled\"");

    let mut graph_str2 = vec![];
    println!("{graph_str}");
    graph_str2.push(graph_str[0..graph_str.find("0 -- 1 [ ]").unwrap()].to_string());
    graph_str2.extend_from_slice(&graph_str_[..]);

    let mut str: String = graph_str2.join("\n");
    str.push_str("\n}");
    println!("{str}");
}
