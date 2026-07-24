use std::collections::HashSet;

fn main() {
    let vec = vec![1,2,6,7,8];

    println!("{}", longest_consecutive(vec));
}

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut hash = HashSet::new();
    let mut count = 1;
    let mut longest_sequence = 0;

    if nums.is_empty(){
        return 0;
    }

    for i in 0..nums.len(){
        hash.insert(nums[i]);
    }

    for i in &hash{
        if !hash.contains(&(i - 1)){
            let mut init = *i;

            while hash.contains(&(init + 1)){
                init+=1;
                count+=1;
            }

            if count > longest_sequence{
                longest_sequence = count;
            }
        }

        count = 1;
    }

    return longest_sequence;        
}