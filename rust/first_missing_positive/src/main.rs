fn main() {
    let vec = vec![1,2,3,4,5];

    println!("{}",first_missing_positive(vec));
}

pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut lesser = 1;
    let mut nums = nums;

    for i in 0..nums.len(){
        while nums[i] > 0 && nums[i] <= nums.len() as i32 && nums[i] != nums[(nums[i] - 1) as usize]{
            let swap = nums[i] - 1;
            nums.swap(i, swap as usize);
        }
    }

    for i in 0..nums.len(){
        if nums[i] == lesser{
            lesser+=1;
        }
    }   
    
    return lesser
}