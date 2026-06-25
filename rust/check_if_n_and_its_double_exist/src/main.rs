use std::collections::HashSet;

fn main() {
    let vec = vec![3,1,7,11];

    println!("{}", check_if_exist(vec));
}

pub fn check_if_exist(arr: Vec<i32>) -> bool {
    let mut hash = HashSet::new();

    for i in arr{
        if hash.contains(&(i*2)){
            return true;
        }

        if i%2 == 0 && hash.contains(&(i/2)){
            return true;
        }

        hash.insert(i);
    }

    return false;
}