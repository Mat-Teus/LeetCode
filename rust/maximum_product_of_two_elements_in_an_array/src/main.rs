fn main() {
    let vec = vec![3,4,5,2];

    println!("{}", max_product(vec));
}

pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut nums = nums;

    nums.sort();

    return (nums[nums.len() - 1] - 1) * (nums[nums.len() - 2] - 1);        
}