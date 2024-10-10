//! Calculate Fibonacci sequence - 1,1,2,3,5,8,13,21,34,55,89..
use std::collections::HashMap;
use std::thread;
use std::time::Instant;

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
pub fn cmp_fibs(n: u64) {
    let mut threads = Vec::new();

    threads.push(thread::spawn(move || {
        let start = Instant::now();
        let res = fib_brutal(n);
        let duration = start.elapsed();
        println!("Time elapsed in `fib_brutal({}) = {}`: {} mil seconds", n, res, duration.as_millis());
    }));

    threads.push(thread::spawn(move || {
        let start = Instant::now();
        let res = fib_with_mem(n, &mut HashMap::<u64, u64>::new());
        let duration = start.elapsed();
        println!("Time elapsed in `fib_with_mem({}) = {}`: {} mil seconds", n, res, duration.as_millis());
    }));

    for t in threads {
        let _ = t.join();
    }
}
