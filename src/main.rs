use std::collections::HashMap;
use crate::fib::{fib, fib_with_mem};

mod fib;

fn main() {
    let res = fib(5);
    println!("{}", res);
    let res = fib_with_mem(50, &mut HashMap::<u64, u64>::new());
    println!("{}", res);
}
