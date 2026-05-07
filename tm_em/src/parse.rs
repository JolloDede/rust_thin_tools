use std::collections::HashMap;

use syn::{
    LitInt, LitStr,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

pub struct GoedelnummerInput {
    pub transitions: Vec<Transition>,
    pub zustaende: Vec<Zustand>,
}

impl GoedelnummerInput {
    pub fn parse_goedel(input: String) -> syn::Result<Self> {
        let mut ch_iter = input.chars().peekable();
        let mut transitions = Vec::new();
        let mut zustaende: HashMap<String, Zustand> = HashMap::new();

        loop {
            let mut start_zustand = None;
            let mut lesen = None;
            let mut end_zustand = None;
            let mut schreiben = None;
            let mut direction = None;

            for i in 0..5 {
                let mut acc = String::new();
                while let Some(ch) = ch_iter.next() {
                    match ch {
                        '1' => {
                            break;
                        }
                        '0' => {
                            acc.push(ch);
                        }
                        _ => {
                            eprintln!("No other symbol other than 0 and 1");
                        }
                    }
                }
                match i {
                    0 | 2 => {
                        let acc_len = acc.len();
                        let mut current_zustand = Zustand(acc_len);
                        if let Some(zustand) = zustaende.get(&acc) {
                            current_zustand = zustand.clone();
                        } else {
                            zustaende.insert(acc, current_zustand.clone());
                        }
                        if i == 0 {
                            start_zustand = Some(current_zustand);
                        } else if i == 2 {
                            end_zustand = Some(current_zustand);
                        }
                    }
                    1 | 3 => {
                        let acc_len = acc.len();
                        if i == 1 {
                            lesen = Some(acc_len);
                        }
                        if i == 3 {
                            schreiben = Some(acc_len);
                        }
                    }
                    4 => {
                        let acc_len = acc.len();
                        match acc_len {
                            1 => direction = Some(Direction::Left),
                            2 => direction = Some(Direction::Right),
                            _ => eprintln!("Failed to read direction"),
                        }
                    }
                    _ => eprintln!("Impossible because of for loop"),
                }
            }

            transitions.push(Transition {
                start_zustand: start_zustand.expect("Failed to read transition start"),
                lesen: lesen.expect("Failed to read transition lesen"),
                end_zustand: end_zustand.expect("Failed to read transition end"),
                schreiben: schreiben.expect("Failed to read transition schreiben"),
                direction: direction.expect("Failed to read transition dir"),
            });

            if let Some(ch) = ch_iter.next() {
                match ch {
                    // Here the are two 1 read so its another transition
                    '1' => {
                        // Here the are three 1 read so its the input after that
                        if Some(&'1') == ch_iter.peek() {
                            break;
                        }
                    }
                    '0' => {
                        eprintln!("There cant be 0 as a splitter of transitions");
                    }
                    _ => {
                        eprintln!("No other symbol other than 0 and 1");
                    }
                }
            }
        }

        let mut zustaende: Vec<Zustand> = zustaende.into_values().collect();
        zustaende.sort();
        Ok(GoedelnummerInput {
            transitions,
            zustaende,
        })
    }
}

#[derive(Clone)]
pub struct Transition {
    pub start_zustand: Zustand,
    pub lesen: usize,
    pub end_zustand: Zustand,
    pub schreiben: usize,
    pub direction: Direction,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Zustand(pub usize);

#[derive(Clone)]
pub enum Direction {
    Left,
    Right,
}
