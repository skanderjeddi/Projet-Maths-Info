use crate::cnf::*;

pub struct SAT {
    pub clauses: Vec<Clause>,
}

impl SAT {
    pub fn new(clauses: Vec<Clause>) -> Self {
        SAT { clauses }
    }

    pub fn evaluate(sat: &SAT, valuations: &[Variable]) -> bool {
        let mut valid = true;
        for clause in &sat.clauses {
            valid = valid && clause.evaluate(&valuations[..]);
        }
        valid
    }

    pub fn has_valid_valuations(sat: &SAT, n: usize) -> bool {
        match SAT::find_one_valid_valuation(sat, n) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn find_one_valid_valuation(sat: &SAT, n: usize) -> Option<Vec<Variable>> {
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

    pub fn find_all_valid_valuations(sat: &SAT, n: usize) -> Vec<Vec<Variable>> {
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

    pub fn collect_literals(&self) -> Vec<String> {
        let mut lits = vec![];
        for c in &self.clauses[..] {
            for l in c.literals {
                if !lits.contains(&l.to_string()) {
                    lits.push(l.to_string());
                }
            }
        }
        lits
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