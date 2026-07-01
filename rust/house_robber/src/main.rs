use std::cmp::max;

fn main() {
    let vec = vec![7];

    println!("{}", rob(vec));
}

pub fn rob(nums: Vec<i32>) -> i32 {
    let mut dp = Vec::new();
    dp.push(nums[0]);

    if nums.len() >= 2 && nums[1] > nums[0]{
        dp.push(nums[1]);
    }else{
        dp.push(nums[0]);
    }

    for i in 2..nums.len(){
        dp.push(max(dp[i - 1], dp[i - 2] + nums[i]));
    }

    return dp[dp.len() - 1];        
}