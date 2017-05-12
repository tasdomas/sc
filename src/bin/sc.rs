extern crate sc;

use sc::stack;

fn main() {
    let s = stack::Stack::<i32>::new();
    
    println!("{}", stack::GREETING);
}
