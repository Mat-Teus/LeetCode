use std::collections::HashSet;

fn main() {
    let vec = vec![-1,2, 1];

    println!("{}", find_max_k(vec));
}

pub fn find_max_k(nums: Vec<i32>) -> i32 {
    let mut hash = HashSet::new();
    let mut bigger = -1;

    for i in 0..nums.len(){
        hash.insert(nums[i]);

        let aux = nums[i] * -1;

        if hash.contains(&aux) && aux.abs() > bigger{
            bigger = aux.abs();
        }
        
    }

    

    return bigger;
}