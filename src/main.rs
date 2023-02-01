// OXY
// Ordinary Xenial Yammerer
use std::fs;
use std::fmt;
use std::process;
use std::env;

use itertools::{
    Itertools,
    EitherOrBoth::*,
};
use num_bigint::BigInt;

enum ExitCode {
    OK,
    NoCommandArguments,
    NoSuchFile,
    TypeMismatch,
}

const CODE_PAGE : &'static [char] = &[
    '…', '‾', '‿', '¡', '¿', '·', '∧', '∨', '≔', '§', '¶', '∀', '∃', '◯', '√', '¬',
    '×', '÷', '≠', '≈', '≤', '≥', '±', '⊓', '⊔', '∈', '∞', '∅', '⊆', '⊇', '∩', '∪',
    ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?',
    '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_',
    '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', '⌂',
    '⁅', '⁆', '⟦', '⟧', '⟨', '⟩', '⟪', '⟫', '⌈', '⌉', '⌊', '⌋', '⦇', '⦈', '«', '»',
    '™', 'Ạ', 'ß', 'Ċ', 'Ď', 'Ẹ', 'Ψ', 'Ġ', 'Ħ', 'İ', 'Ĵ', 'Ķ', 'Ŀ', 'Λ', 'Ŋ', 'Ọ',
    'Þ', 'Ω', 'Ř', 'Ŝ', 'Ť', 'Ụ', 'Δ', 'Ẅ', 'Ξ', 'Ỵ', 'Ż', '∁', '°', '∫', '∏', '∑',
    'θ', 'ạ', 'β', 'ċ', 'đ', 'ė', 'φ', 'ġ', 'ħ', 'ï', 'ĵ', 'ķ', 'ŀ', 'μ', 'ñ', 'ō',
    'π', 'ω', 'ř', 'ŝ', 'ŧ', 'λ', 'μ', 'ŵ', 'χ', 'ÿ', 'ż', '⊢', '⍨', '†', '‡', '↭',
    '𝔽', 'ℕ', 'ℚ', '𝕊', 'ℤ', '⊛', '⊖', '⊗', '⊙', '⊕', '½', '¼', '¾', '↋', '↊', '૪',
    '₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉', '→', '←', '↓', '↑', '↔', '↕',
    '⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹', '⇒', '⇐', '⇓', '⇑', '⇔', '⇕',
];

fn string_to_bytes(s: &str) -> Vec<u8> {
    let mut v = Vec::new();
    for c in s.chars() {
        match CODE_PAGE.iter().position(|&ch| ch == c) {
            None => {
                if c == '\n' || c == '\t' {
                    v.push(c as u8);
                }
            },
            Some(n) => v.push(n as u8)
        }
    }
    return v;
}

fn bytes_to_ops(v: &Vec<u8>) -> Vec<char> {
    return v
        .iter()
        .map(|&x| CODE_PAGE[x as usize])
        .collect();
}

#[derive(Clone, Debug)]
struct OxyToken {
    ops : Vec<char>,
    children: Vec<OxyToken>,
}

const OPENERS : &'static [char] = &['Λ', '†', '‡'];

fn tokenize(v: Vec<char>) -> Vec<OxyToken> {
    let mut tokens_stack : Vec<(char, Vec<OxyToken>)> = vec![];
    let mut tokens : Vec<OxyToken> = vec![];
    for ch in v {
        if OPENERS.contains(&ch) {
            tokens_stack.push((ch, tokens));
            tokens = vec![];
        }
        else if ch == '°' {
            match tokens_stack.pop() {
                Some((src, tks)) => {
                    let next_token = OxyToken {
                        ops: vec![ src ],
                        children: tokens
                    };
                    tokens = tks;
                    tokens.push(next_token);
                },
                None => panic!("Unexpected closing")
            };
        }
        else {
            tokens.push(OxyToken {
                ops: vec![ ch ],
                children: vec![]
            });
        }
    }
    loop {
        match tokens_stack.pop() {
            Some((src, tks)) => {
                let next_token = OxyToken {
                    ops: vec![ src ],
                    children: tokens
                };
                tokens = tks;
                tokens.push(next_token);
            },
            None => break,
        }
    }

    return tokens;
}

fn range_inclusive(left: BigInt, right: BigInt) -> Vec<BigInt> {
    let mut res = vec![];
    let mut it = left;
    while it <= right {
        res.push(it.clone());
        it += 1;
    }
    return res;
}

#[derive(Clone)]
enum Atom {
    Big(BigInt),
    List(Vec<Atom>)
}

impl fmt::Display for Atom {
    // uses sign + for show type
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.sign_plus() {
            match self {
                Atom::Big(_) => write!(f, "int"),
                Atom::List(_) => write!(f, "list"),
            }
        }
        else {
            match self {
                Atom::Big(b) => write!(f, "{}", b),
                Atom::List(v) => {
                    write!(f, "[ ")?;
                    for el in v {
                        write!(f, "{} ", el)?;
                    }
                    write!(f, "]")?;
                    Ok(())
                }
            }
        }
    }
}

struct OxyState {
    stack: Vec<Atom>,
    ops: Vec<OxyToken>,
}

impl OxyState {
    pub fn new(ops: Vec<OxyToken>) -> Self {
        Self {
            stack: vec![],
            ops,
        }
    }

    fn pop(&mut self) -> Atom {
        return match self.stack.pop() {
            Some(val) => val,
            // TODO: implicit input
            None => Atom::Big(BigInt::from(42)),
        };
    }

    fn mismatch<T: std::fmt::Display>(&mut self, op: T, args: &[&Atom]) -> Atom {
        eprint!("Type Mismatch in {op}: No case for ( ");
        for a in args {
            eprint!("{:+} ", a);
        }
        eprintln!(")");
        process::exit(ExitCode::TypeMismatch as i32);
    }

    fn vectorize_dyad(&mut self, ops: Vec<OxyToken>, left: &Atom, right: &Atom) -> Atom {
        match (left, right) {
            (Atom::List(lvec), Atom::List(rvec)) => {
                let list = lvec.iter()
                    .zip(rvec.iter())
                    .map(|(x, y)| {
                        let mut inner = OxyState::new(ops.clone());
                        inner.stack = vec![ x.clone(), y.clone() ];
                        inner.run();
                        inner.pop()
                    })
                    .collect();
                return Atom::List(list);
            },
            (Atom::List(lvec), right) => {
                let list = lvec.iter()
                    .map(|x| {
                        let mut inner = OxyState::new(ops.clone());
                        inner.stack = vec![ x.clone(), right.clone() ];
                        inner.run();
                        inner.pop()
                    })
                    .collect();
                return Atom::List(list);
            }
            (left, Atom::List(rvec)) => {
                let list = rvec.iter()
                    .map(|y| {
                        let mut inner = OxyState::new(ops.clone());
                        inner.stack = vec![ left.clone(), y.clone() ];
                        inner.run();
                        inner.pop()
                    })
                    .collect();
                return Atom::List(list);
            }
            (a, b) => self.mismatch(format!("Vectorized"), &[&a, &b]),
        }
    }

    pub fn exec(&mut self, token: OxyToken) {
        match token.ops[..] {
            // push digit
            [dig @ '0'..='9'] =>
                self.stack.push(Atom::Big(BigInt::from(dig as u32 - '0' as u32))),
            // digit extension
            [dig @ '₀'..='₉'] => {
                self.stack.push(Atom::Big(BigInt::from(10)));
                self.exec(OxyToken { ops: vec!['×'], children: vec![] });
                if dig != '₀' {
                    self.stack.push(Atom::Big(BigInt::from(dig as u32 - '₀' as u32)));
                    self.exec(OxyToken { ops: vec!['+'], children: vec![] });
                }
            },
            // singleton
            ['¡'] => {
                let popped = self.pop();
                self.stack.push(Atom::List(vec![popped]));
            },
            // pair
            ['‿'] => {
                let b = self.pop();
                let a = self.pop();
                self.stack.push(Atom::List(vec![a, b]));
            },
            // wrap stack
            ['‾'] => {
                let wrapped = Atom::List(self.stack.clone());
                self.stack = vec![wrapped];
            },
            // unwrap stack
            ['…'] => {
                match self.pop() {
                    Atom::List(v) => {
                        for el in v {
                            self.stack.push(el);
                        }
                    },
                    v => { self.mismatch("…", &[&v]); },
                }
            },
            // swap top two
            ['⍨'] => {
                let first = self.pop();
                let second = self.pop();
                self.stack.push(first);
                self.stack.push(second);
            },
            // concatenate
            ['c'] => {
                let b = self.pop();
                let a = self.pop();
                let concatenated = match (a, b) {
                    (Atom::List(left), Atom::List(right)) =>
                        Atom::List([left, right].concat()),
                    (Atom::List(left), right) =>
                        Atom::List([left, vec![right]].concat()),
                    (left, Atom::List(right)) =>
                        Atom::List([vec![left], right].concat()),
                    (left, right) =>
                        Atom::List(vec![left, right]),
                };
                self.stack.push(concatenated);
            },
            // range
            ['r'] => {
                match self.pop() {
                    Atom::Big(b) => self.stack.push(
                        Atom::List(
                            range_inclusive(BigInt::from(1), b)
                            .into_iter()
                            .map(Atom::Big)
                            .collect()
                        )
                    ),
                    v => { self.mismatch("r", &[&v]); },
                }
            },
            // map
            ['Λ'] => {
                match self.pop() {
                    Atom::List(list) => {
                        let mapped = list.into_iter()
                            .map(|x| {
                                let mut inner = OxyState::new(token.children.clone());
                                inner.stack = vec![x];
                                inner.run();
                                inner.pop()
                            })
                            .collect();
                        self.stack.push(Atom::List(mapped));
                    },
                    v => { self.mismatch("Λ", &[&v]); },
                }
            },
            // fold
            ['†'] => {
                match self.pop() {
                    Atom::List(list) => {
                        let folded = match &list[..] {
                            [] => panic!("Implicit seeds for empty lists unimplemented"),
                            [a] => a.clone(),
                            [head, ref rest @ ..] =>
                                rest.iter()
                                    .fold(head.clone(), |x, y| {
                                        let mut inner = OxyState::new(token.children.clone());
                                        inner.stack = vec![x.clone(), y.clone()];
                                        inner.run();
                                        inner.pop()
                                    })
                                    .clone(),
                        };
                        self.stack.push(folded);
                    },
                    v => { self.mismatch("†", &[&v]); },
                }
            },
            // seeded fold
            ['‡'] => {
                let seed = self.pop();
                match self.pop() {
                    Atom::List(list) => {
                        let folded = list.iter()
                            .fold(seed.clone(), |x, y| {
                                let mut inner = OxyState::new(token.children.clone());
                                inner.stack = vec![x.clone(), y.clone()];
                                inner.run();
                                inner.pop()
                            })
                            .clone();
                        self.stack.push(folded);
                    },
                    v => { self.mismatch("‡", &[&seed, &v]); },
                }
            },
            // add
            ['+'] => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (Atom::Big(left), Atom::Big(right)) => {
                        let sum = left + right;
                        self.stack.push(Atom::Big(sum));
                    },
                    // vectorize cases
                    (left, right) => {
                        let list = self.vectorize_dyad(vec![ token ], &left, &right);
                        self.stack.push(list);
                    },
                }
            },
            // subtract
            ['-'] => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (Atom::Big(left), Atom::Big(right)) => {
                        let diff = left - right;
                        self.stack.push(Atom::Big(diff));
                    },
                    // vectorize cases
                    (left, right) => {
                        let list = self.vectorize_dyad(vec![ token ], &left, &right);
                        self.stack.push(list);
                    },
                }
            },
            // multiply
            ['×'] => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (Atom::Big(left), Atom::Big(right)) => {
                        let prod = left * right;
                        self.stack.push(Atom::Big(prod));
                    },
                    // vectorize cases
                    (left, right) => {
                        let list = self.vectorize_dyad(vec![ token ], &left, &right);
                        self.stack.push(list);
                    },
                }
            },
            // divide
            ['÷'] => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (Atom::Big(left), Atom::Big(right)) => {
                        let quot = left / right;
                        self.stack.push(Atom::Big(quot));
                    },
                    // vectorize cases
                    (left, right) => {
                        let list = self.vectorize_dyad(vec![ token ], &left, &right);
                        self.stack.push(list);
                    },
                }
            },
            // interleave
            ['↓'] => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (Atom::List(left), Atom::List(right)) => {
                        let zipped = left.into_iter()
                            .zip_longest(right.into_iter())
                            .flat_map(|pair| match pair {
                                Both(l, r) => vec![l, r],
                                Left(l) => vec![l],
                                Right(r) => vec![r],
                            })
                            .collect();
                        self.stack.push(Atom::List(zipped));
                    },
                    (left, right) => { self.mismatch("↓", &[&left, &right]); },
                }
            },
            // uninterleave
            ['↑'] => {
                match self.pop() {
                    Atom::List(v) => {
                        let first = v.iter().step_by(2).cloned().collect();
                        let second = v.iter().skip(1).step_by(2).cloned().collect();
                        self.stack.push(Atom::List(first));
                        self.stack.push(Atom::List(second));
                    },
                    v => { self.mismatch("↑", &[&v]); },
                }
            },
            // exit
            ['·'] => {
                process::exit(ExitCode::OK as i32);
            },
            _ => {},
        }
    }

    pub fn exec_sequence(&mut self, ops: Vec<OxyToken>) {
        for op in ops {
            self.exec(op);
        }
    }

    pub fn run(&mut self) {
        self.exec_sequence(self.ops.clone());
    }
    
    pub fn debug(self) {
        for el in &self.stack {
            println!("{}", el.to_string());
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("Error: Expected at least 1 argument");
        eprintln!("Usage: {} file-name", args[0]);
        process::exit(ExitCode::NoCommandArguments as i32);
    }

    let file_path = &args[1];

    let contents = match fs::read_to_string(file_path) {
        Err(e) => {
            eprintln!("Error: Could not read file: {e}");
            process::exit(ExitCode::NoSuchFile as i32);
        },
        Ok(s) => s
    };

    let bytes = string_to_bytes(&contents);

    eprintln!("Bytes: {}", bytes.len());
    eprintln!("{:?}", bytes);

    let ops = bytes_to_ops(&bytes);
    let tokens = tokenize(ops);
    let mut state = OxyState::new(tokens);
    state.run();
    state.debug();
}
