//! if we can take numbers from a collection of numbers to add up to the target
//! we may use an element of the collection multiple times or zero time
//! all input numbers are non-negative

use std::thread;
use std::time::Instant;

// clone whenever is possible to avoid lifetime error
pub fn can_sum_brutal(target: i32, nums: Vec<u32>) -> bool {
    if target == 0 {
        return true;
    }

    if target < 0 {
        return false;
    }

    for num in nums.clone() {
        let remainder = target - num as i32;
        if can_sum_brutal(remainder, nums.clone()) {
            return true;
        }
    }
    false
}

pub fn cmp_can_sum(target: i32, nums: Vec<u32>) {
    let mut threads = Vec::new();

    threads.push(thread::spawn(move || {
        let start = Instant::now();
        let res = can_sum_brutal(target, nums.clone());
        let duration = start.elapsed();
        println!("Time elapsed in `can_sum_brutal({}, {:?}) = {}`: {} mil seconds", target, nums, res, duration.as_millis());

    }));
}
mod tests {
    use super::{can_sum_brutal};

    #[test]
    fn test_can_sum_brutal_true() {
        assert!(can_sum_brutal(5,  vec![1,2,3]));
    }

    #[test]
    fn test_can_sum_brutal_false() {
        assert!(!can_sum_brutal(7,  vec![2,4]));
    }
}