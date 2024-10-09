use std::collections::HashMap;
use std::time::Instant;
use std::thread;
use crate::fib::{fib_brutal, fib_with_mem};
use crate::grid_traveler::{grid_traveler_brutal, grid_traveler_with_mem};

mod fib;
mod grid_traveler;

fn cmp_fibs(n: u64) {
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

fn cmp_grid_traveler(m:u64, n: u64) {
    let mut threads = Vec::new();

    threads.push(thread::spawn(move || {
        let start = Instant::now();
        let res = grid_traveler_brutal(m, n);
        let duration = start.elapsed();
        println!("Time elapsed in `grid_traveler_brutal({}, {}) = {}`: {} mil seconds", m, n, res, duration.as_millis());
    }));

    threads.push(thread::spawn(move || {
        let start = Instant::now();
        let res = grid_traveler_with_mem(m, n, &mut HashMap::<String, u64>::new());
        let duration = start.elapsed();
        println!("Time elapsed in `grid_traveler_with_mem({}, {}) = {}`: {} mil seconds", m, n, res, duration.as_millis());
    }));

    for t in threads {
        let _ = t.join();
    }
}
fn main() {
    let mut threads = Vec::new();

    threads.push(thread::spawn(move || {
        cmp_fibs(25);
    }));
    threads.push(thread::spawn(move || {
        cmp_grid_traveler(18, 18);
    }));

    for t in threads {
        let _ = t.join();
    }
}
