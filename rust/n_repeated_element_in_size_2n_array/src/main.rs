fn main() {
    let vec = vec![1,2,3,3];

    println!("{}", repeated_n_times(vec));
}

pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();

    for i in 0..nums.len(){
        if nums[i] == nums[i+1]{
            return nums[i];
        }
    }

    return 0;
}