use std::collections::HashSet;

fn main() {
    let vec = vec![5,3,6,1,12];
    let target = 4;

    println!("{}", find_final_value(vec, target));
}

pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
    let mut hash = HashSet::new();
    let mut solution = 0;

    for i in nums{
        hash.insert(i);
    }

    println!("{:?}", hash);

    if hash.contains(&original){
        solution = original;
    }

    while hash.contains(&solution){
        if hash.contains(&solution){
            solution = solution * 2;
        }
    }

    return solution;
}