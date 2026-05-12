use std::collections::HashSet;

fn main() {
    let vec = vec![1, 2, 3, 1];

    println!("{}", contains_duplicate(vec));
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut list = HashSet::new();

    for i in nums{
        if !list.insert(i){
            return true;
        }
    }

    return false;
}