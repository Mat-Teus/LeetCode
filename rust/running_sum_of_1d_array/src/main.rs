fn main() {
    let vec = vec![3,1,2,10,1];

    println!("{:?}", running_sum(vec));
}

pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut solution = vec![0; nums.len()];

    solution[0] = nums[0];

    for i in 1..nums.len(){
        solution[i] = solution[i - 1] + nums[i];
    }

    return solution;
}