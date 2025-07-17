use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&j) = seen.get(&complement) {
                return vec![j as i32, i as i32];
            }
            seen.insert(num, i);
        }
        unreachable!()
    }
}

fn main() {
    let result = Solution::two_sum(vec![3,4,5,6], 9);
    println!("{:?}",result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![0,1]);
        assert_eq!(Solution::two_sum(vec![3,2,4], 6), vec![1,2]);
        assert_eq!(Solution::two_sum(vec![3,3], 6), vec![0,1]);
    }
}

