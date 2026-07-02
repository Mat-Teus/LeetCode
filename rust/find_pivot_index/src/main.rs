fn main() {
    let vec = vec![5, 1, 2, 2];

    println!("{}", pivot_index(vec));
}

pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut left = vec![0;nums.len()];
    let mut right = vec![0;nums.len()];

    left[0] = nums[0];
    right[nums.len() - 1] = nums[nums.len() - 1];

    for i in 1..nums.len(){
        left[i] = left[i - 1] + nums[i];
    }

    let total = left[nums.len() - 1];

    for i in 0..nums.len() {
        if 2 * left[i] == total + nums[i] {
            return i as i32;
        }
    }

    return -1;        
}