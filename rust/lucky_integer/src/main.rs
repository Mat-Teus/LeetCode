use std::collections::HashMap;

fn main() {
    let vec = vec![2,2,3,3,3];

    println!("{}", find_lucky(vec));
}

pub fn find_lucky(arr: Vec<i32>) -> i32 {
    let mut hash = HashMap::new();
    let mut bigger = -1;

    for num in arr{
        *hash.entry(num).or_insert(0) += 1;
    }

    for (number, freq) in hash{
        if number == freq && number > bigger{
            bigger = number;
        }
    }

    return bigger;
}