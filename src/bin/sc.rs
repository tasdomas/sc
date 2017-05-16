extern crate sc;

use std::io;

use sc::stack;
use sc::ops;

fn main() {
    let mut s = stack::Stack::<f32>::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().as_ref() {
            "q" => break, // exit the stack calculator.
            "+" => { // sum two top-most elements.
                s = ops::Sum(s);
                let v = s.peek();
                match v {
                    Some(value) => println!("={}", value),
                    None => break,
                }
            },
            _ => { // check if a valid value was put in.
                let num = input.trim().parse();
                match num {
                    Ok(val) => s = s.push(val),
                    Err(why) => println!("Unexpected input ({})", why),
                }
            },
        }
    }
    // On exit - pop all elements from the stack.
    loop {
        let (ss, val) = s.pop();
        match val {
            Some(value) => println!("{}", value),
            None => break,
        }
        s = ss;
    }
}
