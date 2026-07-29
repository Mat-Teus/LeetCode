use std::cmp::max;

fn main() {
    let vec = vec![5,4,-1,7,8];

    println!("{}", max_sub_array(vec));
}

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut sum = nums[0];
    let mut answer = nums[0];

    for i in 1..nums.len(){
        let prev = sum;

        sum = max(nums[i], nums[i] + prev);

        answer = max(answer, sum);
    }

    return answer;
}