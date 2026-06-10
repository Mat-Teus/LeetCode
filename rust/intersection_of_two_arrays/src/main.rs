use std::collections::HashSet;

fn main() {
    let vec1 = vec![1,2,3,4,4];
    let vec2 = vec![4,5,6];

    println!("{:?}", intersection(vec1, vec2));
}

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut solution = Vec::new();
    let mut hash = HashSet::new();
    let mut hash_solution = HashSet::new();

    for i in 0..nums1.len(){
        hash.insert(nums1[i]);
    }

    for j in 0..nums2.len(){
        if hash.contains(&nums2[j]){
            hash_solution.insert(nums2[j]);
        }
    }

    solution = hash_solution.into_iter().collect();

    return solution;
}