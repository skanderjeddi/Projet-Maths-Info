use std::str::FromStr;

use clap::Parser;
use rand::{self, Rng};

#[derive(Debug, Copy, Clone)]
struct Literal {
    index: usize,
    negated: Negated,
}

#[derive(Debug, Copy, Clone)]
struct Variable {
    index: usize,
    value: bool,
}

#[derive(Debug, Copy, Clone)]
enum Negated {
    YES,
    NO,
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(x{}, {})", self.index + 1, self.value)
    }
}

#[derive(Debug, Copy, Clone)]
struct Clause {
    literals: [Literal; 3],
}

impl Clause {
    fn _new(us: [usize; 3], flips: [Negated; 3]) -> Self {
        Clause {
            literals: [
                Literal {
                    index: us[0],
                    negated: flips[0],
                },
                Literal {
                    index: us[1],
                    negated: flips[1],
                },
                Literal {
                    index: us[2],
                    negated: flips[2],
                },
            ],
        }
    }

    fn random(variables: usize) -> Self {
        let mut literals: [Literal; 3] = [Literal {
            index: 0,
            negated: Negated::NO,
        }; 3];
        let mut used_indices: Vec<usize> = vec![];
        for i in 0..3 {
            literals[i] = Literal {
                index: {
                    let mut n = rand::thread_rng().gen_range(0..variables);
                    while used_indices.contains(&n) {
                        n = rand::thread_rng().gen_range(0..variables);
                    }
                    used_indices.push(n);
                    n
                },
                negated: match rand::thread_rng().gen_bool(0.5) {
                    true => Negated::YES,
                    false => Negated::NO,
                },
            }
        }
        Clause { literals }
    }

    fn evaluate(self, valuations: &[Variable]) -> bool {
        let v1 = match self.literals[0].negated {
            Negated::NO => valuations[self.literals[0].index].value,
            Negated::YES => !valuations[self.literals[0].index].value,
        };
        let v2 = match self.literals[1].negated {
            Negated::NO => valuations[self.literals[1].index].value,
            Negated::YES => !valuations[self.literals[1].index].value,
        };
        let v3 = match self.literals[2].negated {
            Negated::NO => valuations[self.literals[2].index].value,
            Negated::YES => !valuations[self.literals[2].index].value,
        };
        v1 || v2 || v3
    }

    fn to_string(&self) -> String {
        let mut s: String = String::new();
        match self.literals[0].negated {
            Negated::NO => {
                s.push_str("x");
                s.push_str(&(self.literals[0].index.to_string()));
            }
            Negated::YES => {
                s.push_str("!x");
                s.push_str(&(self.literals[0].index.to_string()));
            }
        }
        s.push_str(" || ");
        match self.literals[1].negated {
            Negated::NO => {
                s.push_str("x");
                s.push_str(&(self.literals[1].index.to_string()));
            }
            Negated::YES => {
                s.push_str("!x");
                s.push_str(&(self.literals[1].index.to_string()));
            }
        }
        s.push_str(" || ");
        match self.literals[2].negated {
            Negated::NO => {
                s.push_str("x");
                s.push_str(&(self.literals[2].index.to_string()));
            }
            Negated::YES => {
                s.push_str("!x");
                s.push_str(&(self.literals[2].index.to_string()));
            }
        }
        s
    }
}

struct SAT {
    clauses: Vec<Clause>,
}

impl SAT {
    fn new(clauses: Vec<Clause>) -> Self {
        SAT { clauses }
    }

    fn evaluate(sat: &SAT, valuations: &[Variable]) -> bool {
        let mut valid = true;
        for clause in &sat.clauses {
            valid = valid && clause.evaluate(&valuations[..]);
        }
        valid
    }

    fn has_valid_valuations(sat: &SAT, n: usize) -> bool {
        match SAT::find_one_valid_valuation(sat, n) {
            Some(_) => true,
            None => false,
        }
    }

    fn find_one_valid_valuation(sat: &SAT, n: usize) -> Option<Vec<Variable>> {
        for i in 0..2_u128.pow(n as u32) {
            let mut current_valuation: Vec<Variable> = vec![];
            for j in 0..n {
                current_valuation.push(Variable {
                    index: j as usize,
                    value: (i >> j & 1) != 0,
                });
            }
            if SAT::evaluate(sat, &current_valuation[..]) {
                return Some(current_valuation);
            }
        }
        None
    }

    fn find_all_valid_valuations(sat: &SAT, n: usize) -> Vec<Vec<Variable>> {
        let mut valid_valuations: Vec<Vec<Variable>> = vec![];
        for i in 0..2_u128.pow(n as u32) {
            let mut current_valuation: Vec<Variable> = vec![];
            for j in 0..n {
                current_valuation.push(Variable {
                    index: j as usize,
                    value: (i >> j & 1) != 0,
                });
            }
            if SAT::evaluate(sat, &current_valuation[..]) {
                valid_valuations.push(current_valuation);
            }
        }
        valid_valuations
    }
}

impl std::fmt::Display for SAT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s: String = String::new();
        let mut i = 0;
        for c in &self.clauses {
            if i != 0 {
                s.push_str(" && ");
            }
            s.push_str("(");
            s.push_str(&c.to_string());
            s.push_str(")");
            i += 1;
        }
        write!(f, "{s}")
    }
}

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
