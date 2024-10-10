use std::thread;
use crate::can_sum::cmp_can_sum;
use crate::fib::cmp_fibs;
use crate::grid_traveler::cmp_grid_traveler;

mod fib;
mod grid_traveler;
mod can_sum;


fn main() {
    let mut threads = Vec::new();

    // fibs
    threads.push(thread::spawn(move || {
        cmp_fibs(25);
    }));

    // gir traveller
    threads.push(thread::spawn(move || {
        cmp_grid_traveler(18, 18);
    }));

    // can sum
    let v= vec![7, 14];
    threads.push(thread::spawn(|| {
        cmp_can_sum(300, v);
    }));

    for t in threads {
        let _ = t.join();
    }
}
