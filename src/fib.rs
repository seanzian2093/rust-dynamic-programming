//! Calculate Fibonacci sequence - 1,1,2,3,5,8,13,21,34,55,89..
use std::collections::HashMap;

/// This is very slow -
pub fn fib_brutal(n: u64) -> u64{
    if n < 2 { return 1;}
    fib_brutal(n - 1) + fib_brutal(n - 2)
}
/// Memorization to avoid unnecessary repetition of calculating
pub fn fib_with_mem(n: u64, mem: &mut HashMap<u64, u64>) -> u64{
    if let Some(&res) = mem.get(&n) {
        return res;
    };
    if n < 2 { return 1;}
    let res_1 = fib_with_mem(n - 1, mem);
    let res_2 = fib_with_mem(n - 2, mem);
    mem.insert(n, res_1+ res_2);
    mem.get(&n).unwrap().clone()
}
