use std::str::FromStr;
use clap::Parser;

use pmi3sat::cnf::*;
use pmi3sat::sat::SAT;

#[derive(Debug, Copy, Clone)]
enum Mode {
    OneSolution,
    AllSolutions,
    IsSoluble,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Mode::AllSolutions => write!(f, "all"),
            Mode::OneSolution => write!(f, "one"),
            Mode::IsSoluble => write!(f, "sat"),
        }
    }
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(Mode::AllSolutions),
            "one" => Ok(Mode::OneSolution),
            "sat" => Ok(Mode::IsSoluble),
            _ => Err(s.to_string()),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    clauses: usize,
    #[clap(short, long)]
    variables: usize,
    #[clap(short, long, default_value_t = Mode::AllSolutions)]
    mode: Mode,
}

fn main() {
    let args = Args::parse();
    let mut clauses = vec![];
    for _ in 0..args.clauses {
        clauses.push(Clause::random(args.variables));
    }
    let sat: SAT = SAT::new(clauses);
    println!(
        "Current 3SAT problem in CNF ({} clauses, {} variables):\n\n{sat}\n",
        args.clauses, args.variables
    );
    use std::time::Instant;
    let now = Instant::now();
    let elapsed: std::time::Duration;
    match args.mode {
        Mode::IsSoluble => {
            let satisfiable = SAT::has_valid_valuations(&sat, args.variables);
            match satisfiable {
                true => println!("Current system is satisfiable."),
                false => println!("Current system is not satisfiable"),
            }
            elapsed = now.elapsed();
        }
        Mode::OneSolution => {
            let v = SAT::find_one_valid_valuation(&sat, args.variables);
            elapsed = now.elapsed();
            match v {
                Some(vs) => {
                    println!("\nValuation:");
                    for v in vs {
                        println!("{}", v);
                    }
                }
                None => println!("No working valuations found."),
            }
        }
        Mode::AllSolutions => {
            let v = SAT::find_all_valid_valuations(&sat, args.variables);
            elapsed = now.elapsed();
            match v.len() {
                0 => println!("No working valuations found."),
                _ => {
                    let mut i = 0;
                    for vs in v {
                        println!("\nValuation #{i}:");
                        for vss in vs {
                            println!("{}", vss);
                        }
                        i += 1;
                    }
                }
            }
        }
    }
    println!(
        "Lookup took {:.2?}ns or {:.8?}s.",
        elapsed.as_nanos(),
        elapsed.as_secs_f64()
    );
}
