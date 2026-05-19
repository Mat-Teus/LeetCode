fn main() {
    let vec = vec![2,3,1,1,4];

    println!("{}", can_jump(vec));
}

pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut max_range = 0;

    for i in 0..nums.len(){
        if i > max_range as usize{
            return false;
        }
        max_range = max_range.max(i as i32 + nums[i]);
        if max_range >= (nums.len() - 1) as i32{
            return true;
        }
    }

    return false;  
}