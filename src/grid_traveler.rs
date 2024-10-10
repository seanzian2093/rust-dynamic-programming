//! Suppose we are a traveler on a 2D grid. We begin in the top-left corner and our goal is to travel
//! to the bottom-right corner. We may only move down or right.
//! In how many ways can you travel to the goal on a grid with dimensions of m * n?

use std::collections::HashMap;
use std::thread;
use std::time::Instant;

pub fn grid_traveler_brutal(m: u64, n: u64) -> u32 {
    // if only one cell, only one way
    if m ==1 && n ==1 {
        return 1;
    };

    // if no cells at all, we say no way
    if m ==0 || n ==0 {
        return 0;
    };

    // if grid is more than 1 row and 1 col
    grid_traveler_brutal(m - 1, n) + grid_traveler_brutal(m , n- 1)
}
pub fn grid_traveler_with_mem(m: u64, n: u64, mem: &mut HashMap<String, u64>) -> u64{
    // Construct a key
    let key = format!("{}-{}", m, n);
    // Find in buffer by the key
    if let Some(&res) = mem.get(&key) {
        return res;
    }

    // Calculate if not in the buffer
    if m ==1 && n ==1 {
        return 1;
    };

    if m ==0 || n ==0 {
        return 0;
    };

    // Update the buffer
    let res1 = grid_traveler_with_mem(m - 1, n, mem);
    let res2 = grid_traveler_with_mem(m , n- 1, mem);
    mem.insert(key, res1 + res2);

    // Return
    res1 + res2
}
pub fn cmp_grid_traveler(m:u64, n: u64) {
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
