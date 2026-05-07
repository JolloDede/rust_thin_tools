use tm_em::turing_machine;

use crate::tm_emulator::{Band, TuringMachine};

mod tm_emulator {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum Direction {
        Left,
        Right,
    }

    #[derive(Clone, Debug)]
    pub struct Transition<S> {
        pub start: S,
        pub read: char,
        pub end: S,
        pub write: char,
        pub direction: Direction,
    }

    #[derive(Clone, Debug)]
    pub struct Band;

    impl Band {
        pub fn new(_input: Vec<char>) -> Self {
            Self
        }
    }

    pub struct TuringMachine<S> {
        pub state: S,
        pub transitions: Vec<Transition<S>>,
        pub band: Band,
    }

    impl<S> TuringMachine<S> {
        pub fn new(transitions: Vec<Transition<S>>, band: Band, state: S) -> Self {
            Self {
                state,
                transitions,
                band,
            }
        }

        pub fn step(&mut self) -> bool {
            false
        }
    }
}

#[test]
fn test() {
    let mut tm = turing_machine!("010010001010011000101010010110001001001010011000100010001010111");

    tm.band = Band::new("111".chars().collect());
    tm.step();
}
