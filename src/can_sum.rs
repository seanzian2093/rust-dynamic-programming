//! if we can take numbers from a collection of numbers to add up to the target
//! we may use an element of the collection multiple times or zero time
//! all input numbers are non-negative

use std::collections::HashMap;
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

pub fn can_sum_with_mem(target: i32, nums: Vec<u32>, mem: &mut HashMap<i32, bool>) -> bool {
    if let Some(&res) = mem.get(&target) {
        return res;
    }
    if target == 0 {
        return true;
    }

    if target < 0 {
        return false;
    }

    for num in nums.clone() {
        let remainder = target - num as i32;
        if can_sum_with_mem(remainder, nums.clone(), mem) {
            mem.insert(target, true);
            return true;
        }
    }
    mem.insert(target, false);
    false
}

pub fn cmp_can_sum(target: i32, nums: Vec<u32>) {
    let mut threads = Vec::new();

    let nums_clone = nums.clone();
    threads.push(thread::spawn(move|| {
        let start = Instant::now();
        let res = can_sum_brutal(target, nums_clone.clone());
        let duration = start.elapsed();
        println!("Time elapsed in `can_sum_brutal({}, {:?}) = {}`: {} mil seconds", target, nums_clone, res, duration.as_millis());

    }));

    let nums_clone = nums.clone();
    threads.push(thread::spawn(move || {
        let start = Instant::now();
        let res = can_sum_with_mem(target, nums_clone.clone(), &mut HashMap::<i32, bool>::new());
        let duration = start.elapsed();
        println!("Time elapsed in `can_sum_with_mem({}, {:?}) = {}`: {} mil seconds", target, nums_clone, res, duration.as_millis());

    }));

    for t in threads {
        let _ = t.join();
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::{can_sum_brutal, can_sum_with_mem};

    #[test]
    fn test_can_sum_brutal_true() {
        assert!(can_sum_brutal(5,  vec![1,2,3]));
    }

    #[test]
    fn test_can_sum_mem_true() {
        assert!(can_sum_with_mem(5,  vec![1,2,3], &mut HashMap::<i32, bool>::new()));
    }

    #[test]
    fn test_can_sum_brutal_false() {
        assert!(!can_sum_brutal(7,  vec![2,4]));
    }

    #[test]
    fn test_can_sum_mem_false() {
        assert!(!can_sum_with_mem(7,  vec![2,4], &mut HashMap::<i32, bool>::new()));
    }

}