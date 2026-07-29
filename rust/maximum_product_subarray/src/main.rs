use std::cmp::{max, min};

fn main() {
    let vec = vec![-2,-3,7];

    println!("{}", max_product(vec));
}

pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut max_value = nums[0];
    let mut min_value = nums[0];
    let mut answer = nums[0];

    for i in 1..nums.len(){
        let prev = max_value;
        let prev_min = min_value;

        max_value = max(nums[i], max(nums[i] * prev_min, nums[i] * prev));
        min_value = min(nums[i], min(nums[i] * prev, nums[i] * prev_min));
        
        answer = max(answer, max_value);
    }

    return answer;        
}