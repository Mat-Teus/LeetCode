fn main() {
    let vec = vec![-1,1,0,-3,3];

    println!("{:?}", product_except_self(vec));
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut solution = vec![0;nums.len()];
    let mut suffix = vec![0;nums.len()];

    solution[0] = 1;
    suffix[nums.len() - 1] = 1;

    for i in 1..nums.len(){
        solution[i] += nums[i - 1] * solution[i-1];
    }

    for i in (0..nums.len() - 1).rev(){
        suffix[i] = nums[i + 1] * suffix[i + 1];
    }

    for i in 0..nums.len(){
        solution[i] *= suffix[i];
    }

    return solution;
}