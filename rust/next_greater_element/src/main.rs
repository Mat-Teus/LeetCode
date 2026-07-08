use std::collections::HashMap;

fn main() {
    let vec = vec![1,3,5,2,4];
    let vec2 = vec![6,5,4,3,2,1,7];

    println!("{:?}", next_greater_element(vec, vec2));
}

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut solution = Vec::new();
    let mut hash = HashMap::new();
    let mut stack = Vec::new();


    for i in nums2{
        if !stack.is_empty() && i > *stack.last().unwrap(){
            while !stack.is_empty() && i > *stack.last().unwrap(){
                hash.insert(*stack.last().unwrap(), i);
                stack.pop();
            }
        }

        stack.push(i);
    }

    println!("{:?}", hash);

    for i in 0..nums1.len(){
        if hash.contains_key(&nums1[i]){
            solution.push(*hash.get(&nums1[i]).unwrap());
        }
    }

    return solution;
}