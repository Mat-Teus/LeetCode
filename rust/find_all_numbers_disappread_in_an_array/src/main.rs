use std::collections::HashSet;

fn main() {
    let vec = vec![1,1];

    println!("{:?}", find_disappeared_numbers(vec));
}

pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut solution = Vec::new();
    let mut hash = HashSet::new();
    let n = nums.len();
    
    for i in nums{
        hash.insert(i);
    }

    let mut i = 0;

    while (i as usize) < n{
        if !hash.contains(&(i + 1)){
            solution.push(i + 1);
        }

        i+=1;
    }

    return solution;
}