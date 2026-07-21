use std::collections::HashMap;

fn main() {
    let vec = vec![1,0,1,1];
    let k = 1;

    println!("{}", contains_nearby_duplicate(vec, k));
}

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut hash = HashMap::new();
    
    for i in 0..nums.len(){
        if hash.contains_key(&nums[i]){
            let index = hash.get(&nums[i]).unwrap();
            if &i - index <= k as usize{
                return true;
            }
        }

        hash.insert(nums[i], i);
    }

    return false;        
}