fn main() {
    let vec = vec![1,2,3];

    println!("{}", maximum_product(vec));
}

pub fn maximum_product(nums: Vec<i32>) -> i32 { 
    let mut nums = nums; 
    nums.sort(); 
    if nums[nums.len() - 1] * nums[nums.len() - 2] * nums[nums.len() - 3] > nums[0] * nums[1] * nums[nums.len() - 1]{ 
        return nums[nums.len() - 1] * nums[nums.len() - 2] * nums[nums.len() - 3] 
    }else{ 
        return nums[0] * nums[1] * nums[nums.len() - 1] 
    } 
}