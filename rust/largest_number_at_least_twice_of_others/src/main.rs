fn main() {
    let vec = vec![1,0];

    println!("{}", dominant_index(vec));
}

pub fn dominant_index(nums: Vec<i32>) -> i32 {
    let mut bigger = 0;
    let mut second = 0;

    for i in 0..nums.len(){
        if nums[i] > nums[bigger as usize]{
            second = bigger;
            bigger = i as i32;
        }else if nums[i] > nums[second as usize] || second == bigger{
            second = i as i32;
        }
    }
    
    if nums[bigger as usize] >= nums[second as usize] * 2{
        return bigger;
    }else{
        return -1;
    }
}