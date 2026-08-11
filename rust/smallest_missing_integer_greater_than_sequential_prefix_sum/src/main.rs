use std::collections::HashSet;

fn main() {
    let vec = vec![46,8,2,4,1,4,10,2,4,10,2,5,7,3,1];

    println!("{}", missing_integer(vec));
}

pub fn missing_integer(nums: Vec<i32>) -> i32 {
    let mut sum = nums[0];
    let mut hash = HashSet::new();
    let mut is_seq = true;

    hash.insert(nums[0]);

    for i in 1..nums.len(){
        hash.insert(nums[i]);

        if nums[i] == nums[i - 1] + 1 && is_seq == true{
            sum+=nums[i];
        }else{
            is_seq = false;
        }
    }

    let mut answer = sum;

    while hash.contains(&answer){
        answer+=1;
    }


    return answer;
}