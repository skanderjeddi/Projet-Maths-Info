use rand::{self, Rng};

#[derive(Debug, Copy, Clone)]
pub struct Literal {
    pub index: usize,
    pub negated: Negated,
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "x{}", self.index + 1)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Variable {
    pub index: usize,
    pub value: bool,
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(x{}, {})", self.index + 1, self.value)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Negated {
    YES,
    NO,
}

#[derive(Debug, Copy, Clone)]
pub struct Clause {
    pub literals: [Literal; 3],
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

    // pub fn collect_literals() -> &'staticlopm: Vec<(String, String)> { }

    pub fn random(variables: usize) -> Self {
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

    pub fn evaluate(self, valuations: &[Variable]) -> bool {
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

    pub fn to_string(&self) -> String {
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