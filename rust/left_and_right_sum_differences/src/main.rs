fn main() {
    let vec = vec![1];

    println!("{:?}", left_right_difference(vec));
}

pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
    let mut right_sum = Vec::new();
    let mut left_sum = Vec::new();
    let mut sum = 0;

    for i in 0..nums.len(){
        for j in i+1..nums.len(){
            sum+=nums[j];
        }
        right_sum.push(sum);
        sum = 0;
    }

    let mut nums:Vec<i32> = nums;
    nums.reverse();

    for i in 0..nums.len(){
        for j in i+1..nums.len(){
            sum+=nums[j];
        }
        left_sum.push(sum);
        sum = 0;
    }

    left_sum.reverse();
    let mut solution = Vec::new();

    for i in 0..nums.len(){
        solution.push((left_sum[i] - right_sum[i]).abs());
    }

    return solution;
}