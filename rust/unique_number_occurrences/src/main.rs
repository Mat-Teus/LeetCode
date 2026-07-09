use std::collections::HashMap;

fn main() {
    let vec = vec![1,2];

    println!("{}", unique_occurrences(vec));
}

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut hash = HashMap::new();

    for i in arr{
        *hash.entry(i).or_insert(0) += 1;
    }

    let mut aux = Vec::new();

    for (_i,j) in hash{
        aux.push(j);
    }

    aux.sort();

    for i in 0..aux.len() - 1{
        if aux[i] == aux[i + 1]{
            return false;
        }
    }

    return true;        
}