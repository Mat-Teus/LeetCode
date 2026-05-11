fn main() {
    let nums = vec![3,3];
    let target: i32 = 6;

    println!("{:?}", two_sum(nums, target));
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: i32 = 0;
    let mut solution: Vec<i32> = Vec::new();
    let mut index2 = 1;

    for i in 0..nums.len() - 1 {
        for j in index2..nums.len(){
            result = nums[i] + nums[j];
            if result == target{
                solution.push((i) as i32);
                solution.push((j) as i32);
            }
        }

        index2 += 1;
    }

    return solution;
}