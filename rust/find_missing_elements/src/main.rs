use std::collections::HashSet;

fn main() {
    let vec = vec![1,4,2,5];

    println!("{:?}", find_missing_elements(vec));
}

pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut solution = Vec::new();
    let mut first = i32::MAX;
    let mut last = i32::MIN;
    let mut hash = HashSet::new();

    for i in nums{
        if i < first{
            first = i;
        }

        if i > last{
            last = i;
        }

        hash.insert(i);
    }

    for i in first..=last{
        if !hash.contains(&i){
            solution.push(i);
        }
    }

    return solution;        
}