use std::env;

mod stack;
mod tm;

enum Modes {
    Stack,
    TM,
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
        None => println!("Try using any of the Modes: stack, tm"),
    }
}
