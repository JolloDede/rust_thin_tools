use std::env;

mod stack;
mod tm;
mod tm_emulator;

enum Modes {
    Stack,
    TM,
    Emulator,
}

fn main() {
    let mut step_mode = false;
    let mut mode: Option<Modes> = None;
    for arg in env::args() {
        match arg.to_lowercase().as_str() {
            "stack" => {
                mode = Some(Modes::Stack);
            }
            "-s" => {
                step_mode = true;
            }
            "tm" => {
                mode = Some(Modes::TM);
            }
            "emulator" => {
                mode = Some(Modes::Emulator);
            }
            _ => {}
        }
    }
    match mode {
        Some(Modes::Stack) => {
            stack::start(step_mode);
        }
        Some(Modes::TM) => {
            tm::start();
        }
        Some(Modes::Emulator) => {
            tm_emulator::start(step_mode);
        }
        None => println!("Try using any of the Modes: stack, tm"),
    }
}
