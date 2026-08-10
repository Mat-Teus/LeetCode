use std::collections::HashMap;

fn main() {
    let vec = vec![1,2];

    println!("{:?}", majority_element(vec));
}

pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let mut solution = Vec::new();
    let n:f64 = nums.len() as f64/3.0;
    let mut hash = HashMap::new();

    for i in nums{
        *hash.entry(i).or_insert(0) += 1;
    }

    for (i,j) in hash{
        if j as f64 > n{
            solution.push(i);
        }
    }

    return solution;        
}