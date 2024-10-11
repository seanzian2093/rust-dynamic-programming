//! if we can find numbers from a collection of numbers to add up to the target
//! we may use an element of the collection multiple times or zero time
//! all input numbers are non-negative

use std::collections::HashMap;
use std::thread;
use std::time::Instant;

// clone whenever is possible to avoid lifetime error
pub fn how_sum_brutal(target: i32, nums: Vec<u32>) -> Option<Vec<u32>> {
    if target == 0 {
        return Some(Vec::<u32>::new());
    }

    if target < 0 {
        return None;
    }

    for num in nums.clone() {
        let remainder = target - num as i32;
        let res0 = how_sum_brutal(remainder, nums.clone());
        if res0.is_some() {
            let mut res = res0.unwrap();
            res.push(num);
            return Some(res)
        }
    };
    None
}

pub fn how_sum_with_mem(target: i32, nums: Vec<u32>, mem: &mut HashMap<i32, Option<Vec<u32>>>) -> Option<Vec<u32>> {
    if let Some(&ref res) = mem.get(&target) {
        return res.to_owned();
    }

    if target == 0 {
        return Some(Vec::<u32>::new());
    }

    if target < 0 {
        return None;
    }

    for num in nums.clone() {
        let remainder = target - num as i32;
        let res0 = how_sum_with_mem(remainder, nums.clone(), mem);
        if res0.is_some() {
            let mut res = res0.unwrap();
            res.push(num);
            mem.insert(target, Some(res.clone()));
            return Some(res);
        }
    };
    mem.insert(target, None);
    None
}

pub fn cmp_how_sum(target: i32, nums: Vec<u32>) {
    let mut threads = Vec::new();

    let nums_clone = nums.clone();
    threads.push(thread::spawn(move|| {
        let start = Instant::now();
        let res = how_sum_brutal(target, nums_clone.clone());
        let duration = start.elapsed();
        println!("Time elapsed in `how_sum_brutal({}, {:?}) = {:?}`: {} mil seconds", target, nums_clone, res, duration.as_millis());

    }));

    let nums_clone = nums.clone();
    threads.push(thread::spawn(move || {
        let start = Instant::now();
        let res = how_sum_with_mem(target, nums_clone.clone(), &mut HashMap::<i32, Option<Vec<u32>>>::new());
        let duration = start.elapsed();
        println!("Time elapsed in `how_sum_with_mem({}, {:?}) = {:?}`: {} mil seconds", target, nums_clone, res, duration.as_millis());

    }));

    for t in threads {
        let _ = t.join();
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::{how_sum_brutal, how_sum_with_mem};

    #[test]
    fn test_how_sum_brutal_some() {
        let res = how_sum_brutal(7,  vec![5,3,4,7]).unwrap().into_iter().sum::<u32>();
        assert_eq!(res, 7);
    }

    #[test]
    fn test_how_sum_brutal_some2() {
        let res = how_sum_brutal(7,  vec![2,3]).unwrap().into_iter().sum::<u32>();
        assert_eq!(res, 7);
    }

    #[test]
    fn test_how_sum_brutal_some3() {
        let res = how_sum_brutal(8,  vec![2,3,5]).unwrap().into_iter().sum::<u32>();
        assert_eq!(res, 8);
    }

    #[test]
    fn test_how_sum_mem_true() {
        let res = how_sum_with_mem(7,  vec![5,3,4,7], &mut HashMap::<i32, Option<Vec<u32>>>::new()).unwrap().into_iter().sum::<u32>();
        assert_eq!(res, 7);
    }

    #[test]
    fn test_how_sum_brutal_false() {
        let res = how_sum_brutal(7,  vec![2,4]);
        assert!(res.is_none());
    }

    #[test]
    fn test_how_sum_mem_false() {
        let res = how_sum_with_mem(7,  vec![2,4], &mut HashMap::<i32, Option<Vec<u32>>>::new());
        assert!(res.is_none());
    }

}
