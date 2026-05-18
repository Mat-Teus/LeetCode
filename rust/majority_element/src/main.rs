use std::collections::HashMap;

fn main() {
    let vec = vec![1000000000,1000000000,-1000000000,-1000000000,-1000000000];

    println!("{}",majority_element(vec));
}

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut hash = HashMap::new();

    for i in 0..nums.len(){
        if !hash.contains_key(&nums[i]){
            hash.insert(nums[i], 1);
        }else{
            let count = hash.get(&nums[i]).unwrap();
            hash.insert(nums[i], *count + 1);
        }
    }

    let mut majority = 0;
    let mut index = 0;

    for (&i,&j) in &hash{
        if j > majority{
            majority = j;
            index = i;
        }
    }
    
    return index;
}