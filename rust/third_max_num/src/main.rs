use std::collections::HashSet;

fn main() {
    let vec = vec![1,2];

    println!("{}", third_max(vec));
}

pub fn third_max(nums: Vec<i32>) -> i32 {
    let mut hash = HashSet::new();
    let mut aux = Vec::new();

    for i in nums{
        hash.insert(i);
    }

    for j in hash{
        aux.push(j);
    }

    aux.sort();

    if aux.len() > 2{
        return aux[aux.len() - 3];
    }else{
        return aux[aux.len() - 1];
    }
}