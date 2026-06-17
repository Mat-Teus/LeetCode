fn main() {
    let vec = vec![1,3];
    println!("{}", find_non_min_or_max(vec));
}

pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
    if nums.len() <= 2{
        return -1;
    }else{
        let mut nums = nums;
        nums.sort();
        return nums[1];
    }
}