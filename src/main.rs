use std::thread;
use crate::can_sum::cmp_can_sum;
use crate::fib::cmp_fibs;
use crate::grid_traveler::cmp_grid_traveler;
use crate::how_sum::cmp_how_sum;

mod fib;
mod grid_traveler;
mod can_sum;
mod how_sum;

fn main() {
    let mut threads = Vec::new();

    // fibs
    threads.push(thread::spawn(move || {
        cmp_fibs(25);
    }));

    // gir traveller
    threads.push(thread::spawn(move || {
        cmp_grid_traveler(8, 8);
    }));

    // can sum
    let v= vec![7, 14];
    threads.push(thread::spawn(move || {
        cmp_can_sum(250, v);
    }));

    // how sum
    let v= vec![7, 14];
    threads.push(thread::spawn(move || {
        cmp_how_sum(250, v);
    }));

    for t in threads {
        let _ = t.join();
    }
}
