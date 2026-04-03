use std::{env, thread::sleep, time::Duration};

use crate::my_stack::MyStack;

mod my_stack;

const INPUT: &str = "3 4 + 6 2 + 8 9 + 4 3 + * * *"; // 6664
// const INPUT: &str = "3 1 + 7 8 + 9 8 7 + 1 2 1 4 + + 7 + + + + + +"; // 58
// const INPUT: &str = "3 4 + *";
// const INPUT: &str = "8 + 9 + 7 * 2 *";

fn main() {
    let mut step_mode = false;
    for arg in env::args() {
        if arg == "-s" {
            step_mode = true;
        }
    }
    println!("Fixed stack");
    fixed_slice(step_mode);
    println!();
    println!("Dynamic stack");
    my_stack(INPUT, step_mode);
}

fn fixed_slice(step_mode: bool) {
    let mut stack: [u32; INPUT.len()] = [0; INPUT.len()];
    let mut stack_index = 0;

    for ch in INPUT.chars() {
        match ch {
            ch if ch.is_numeric() => {
                stack[stack_index] = ch.to_digit(10).unwrap();
                stack_index += 1;
            }
            '+' => {
                if stack_index < 2 {
                    panic!("Verwerfend")
                } else {
                    let left_index = stack_index - 2;
                    let right_index = stack_index - 1;
                    stack[left_index] = stack[left_index] + stack[right_index];
                    stack[right_index] = 0;
                    stack_index -= 1;
                }
            }
            '*' => {
                if stack_index < 2 {
                    panic!("Verwerfend")
                } else {
                    let left_index = stack_index - 2;
                    let right_index = stack_index - 1;
                    stack[left_index] = stack[left_index] * stack[right_index];
                    stack[right_index] = 0;
                    stack_index -= 1;
                }
            }
            ' ' => {
                continue;
            }
            _ => panic!("cant use character \"{ch}\""),
        }
        if step_mode {
            // dbg!(&stack);
            println!("stack: {:?}", stack);
            sleep(Duration::from_secs(2));
        }
    }

    println!("{}", stack[0]);
}

fn my_stack(input: &str, step_mode: bool) {
    let mut stack = MyStack::with_capacity(input.len() / 4);

    for ch in input.chars() {
        match ch {
            ch if ch.is_numeric() => {
                stack.push(ch.to_digit(10).unwrap());
            }
            '+' => {
                if stack.len() < 2 {
                    panic!("Verwerfend")
                } else {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left + right);
                }
            }
            '*' => {
                if stack.len() < 2 {
                    panic!("Verwerfend")
                } else {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left * right);
                }
            }
            ' ' => {
                continue;
            }
            _ => panic!("cant use character \"{ch}\""),
        }
        if step_mode {
            println!("stack: {:?}", stack);
            sleep(Duration::from_secs(2));
        }
    }

    println!("{}", stack.pop().unwrap());
}
