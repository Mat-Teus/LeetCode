use std::collections::HashMap;

fn main() {
    let vec = vec![1,2, 4,3];

    println!("{:?}", find_relative_ranks(vec));
}

pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut hash = HashMap::new();
    let mut solution = Vec::new();
    let mut copy = score.clone();
    copy.sort();
    copy.reverse();

    for i in 0..copy.len(){
        if i == 0{
            hash.insert(copy[i], "Gold Medal".to_string());
        }else if i == 1{
            hash.insert(copy[i], "Silver Medal".to_string());
        }else if i == 2{
            hash.insert(copy[i], "Bronze Medal".to_string());
        }else{
            hash.insert(copy[i], i.to_string());
        }
    }

    for j in score{
        solution.push(hash.get(&j).unwrap().clone());
    }

    return solution;
}