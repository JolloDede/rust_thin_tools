use std::{thread::sleep, time::Duration};

use tm_em::turing_machine;

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
pub struct Band {
    cells: Vec<char>,
    pos: usize,
}

impl Band {
    pub fn new(input: Vec<char>) -> Self {
        let mut cells = vec!['0'; 7];
        cells.extend(input);
        cells.extend(vec!['0'; 7]);

        Self { cells, pos: 7 }
    }

    pub fn read(&self) -> char {
        self.cells[self.pos]
    }

    pub fn write(&mut self, ch: char) {
        self.cells[self.pos] = ch;
    }

    pub fn move_left(&mut self) {
        if self.pos == 0 {
            self.cells.insert(0, '0');
        } else {
            self.pos -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.pos + 1 >= self.cells.len() {
            self.cells.push('0');
        }
        self.pos += 1;
    }
}

pub struct TuringMachine<S> {
    pub state: S,
    pub band: Band,
    transitions: Vec<Transition<S>>,
}

impl<S: Copy + PartialEq> TuringMachine<S> {
    pub fn new(transitions: Vec<Transition<S>>, band: Band, start: S) -> Self {
        Self {
            state: start,
            band,
            transitions,
        }
    }

    pub fn step(&mut self) -> bool {
        let symbol = self.band.read();

        if let Some(transition) = self
            .transitions
            .iter()
            .find(|transition| transition.start == self.state && transition.read == symbol)
        {
            self.band.write(transition.write);
            match transition.direction {
                Direction::Left => self.band.move_left(),
                Direction::Right => self.band.move_right(),
            }
            self.state = transition.end;
            true
        } else {
            false
        }
    }
}

pub fn start(step_mode: bool) {
    let mut tm = turing_machine!("010010001010011000101010010110001001001010011000100010001010111");

    tm.band = Band::new("100".chars().collect());
    loop {
        tm.step();

        if step_mode {
            // dbg!(&tm.band);
            println!("band: {:?}", tm.band);
            sleep(Duration::from_secs(2));
        }
    }
}
