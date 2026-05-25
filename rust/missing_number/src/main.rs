fn main() {
    let vec = vec![1,2,3];

    println!("{}", missing_number(vec));
}

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut lesser = 0;
    let mut nums = nums;

    nums.sort();

    for i in 0..nums.len(){
        if nums[i] == lesser{
            lesser+=1;
        }
    }   
    
    return lesser
}