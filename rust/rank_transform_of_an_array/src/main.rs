use std::collections::HashMap;

fn main() {
    let vec = vec![100,100,100];

    println!("{:?}", array_rank_transform(vec));
}

pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    let mut solution = vec![0;arr.len()];
    let mut aux = arr.clone();
    let mut hash = HashMap::new();

    aux.sort();

    let mut ranks = 1;

    for i in 0..aux.len(){
        if i == 0 || aux[i] != aux[i - 1]{
            hash.insert(aux[i], ranks);
            ranks+=1;
        }
    }

    for i in 0..arr.len(){
        if hash.contains_key(&arr[i]){
            solution[i] = *hash.get(&arr[i]).unwrap();
        }
    }

    return solution;
}