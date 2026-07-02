fn main() {
    let vec = vec![3,2,-1,-4,8];

    println!("{}", find_middle_index(vec));
}

pub fn find_middle_index(nums: Vec<i32>) -> i32 {
    if nums.len() == 1{
        return 0;
    }

    let mut preffix = vec![0;nums.len()];
    let mut suffix = vec![0;nums.len()];

    preffix[0] = nums[0];
    suffix[nums.len() - 1] = nums[nums.len() - 1];

    for i in 1..nums.len(){
        preffix[i] = preffix[i - 1] + nums[i];
    }

    for i in (0..nums.len() - 1).rev(){
        suffix[i] = suffix[i + 1] + nums[i];
    }

    if nums.len() >= 2 && suffix[1] == 0{
        return 0;
    }else if nums.len() >= 2 && preffix[nums.len() - 2] == 0{
        return nums.len() as i32 - 1
    }

    for i in 1..nums.len() - 1{
        if preffix[i - 1] == suffix[i + 1]{
            return i as i32;
        }
    }

    return -1;        
}