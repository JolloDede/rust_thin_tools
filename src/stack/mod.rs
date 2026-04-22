use std::{thread::sleep, time::Duration};

use my_stack::*;

mod my_stack;

const INPUT: &str = "3 4 + 6 2 + 8 9 + 4 3 + * * *"; // 6664
// const INPUT: &str = "3 1 + 7 8 + 9 8 7 + 1 2 1 4 + + 7 + + + + + +"; // 58
// const INPUT: &str = "3 4 + *";
// const INPUT: &str = "8 + 9 + 7 * 2 *";
// const INPUT: &str = "1 2 +";

pub fn start(step_mode: bool) {
    println!("Fixed stack");
    fixed_slice(step_mode);
    println!();
    println!("Dynamic stack");
    my_stack(INPUT, step_mode);
}

fn fixed_slice(step_mode: bool) {
    let mut stack: [u32; INPUT.len()] = [0; INPUT.len()];
    let mut stack_index = 0;
    let mut atleast_one_operator = false;

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
                    atleast_one_operator = true;

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
                    atleast_one_operator = true;

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

    dbg!(stack_index, atleast_one_operator);
    if stack_index == 1 && atleast_one_operator {
        println!("{}", stack[0]);
    } else {
        panic!("Verwerfend");
    }
}

fn my_stack(input: &str, step_mode: bool) {
    let mut stack = MyStack::with_capacity(input.len() / 4);
    let mut atleast_one_operator = false;

    for ch in input.chars() {
        match ch {
            ch if ch.is_numeric() => {
                stack.push(ch.to_digit(10).unwrap());
            }
            '+' => {
                if stack.len() < 2 {
                    panic!("Verwerfend")
                } else {
                    atleast_one_operator = true;

                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left + right);
                }
            }
            '*' => {
                if stack.len() < 2 {
                    panic!("Verwerfend")
                } else {
                    atleast_one_operator = true;

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

    dbg!(stack.len(), atleast_one_operator);
    if stack.len() == 1 && atleast_one_operator {
        println!("{}", stack.pop().unwrap());
    } else {
        panic!("Verwerfend");
    }
}
