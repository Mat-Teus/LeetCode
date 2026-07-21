use std::collections::HashMap;

fn main() {
    let vec = vec![1,2,1,2,1,2,3,1,3,2];
    let k = 2;

    println!("{:?}", top_k_frequent(vec, k));
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut solution = Vec::new();
    let mut k = k;
    let mut hash = HashMap::new();

    for i in 0..nums.len(){
        *hash.entry(nums[i]).or_insert(0) += 1;
    }

    while k > 0{
        let mut bigger = 0;
        let mut bigger_index = 0;
        for (i, j) in &hash {
            if *j > bigger {
                bigger = *j;
                bigger_index = *i;
            }
        }
        solution.push(bigger_index);
        hash.remove(&bigger_index);
        k-=1;
    }

    return solution;        
}