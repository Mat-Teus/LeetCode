use std::collections::HashSet;

fn main() {
    let jewels = String::from("z");
    let stones = String::from("ZZ");
    println!("{}", num_jewels_in_stones(jewels, stones));
}

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut hash = HashSet::new();
    let mut count = 0;
    let jewels:Vec<char> = jewels.chars().collect();
    let stones:Vec<char> = stones.chars().collect();

    for i in jewels{
        hash.insert(i);
    }

    for i in stones{
        if hash.contains(&i){
            count+=1;
        }
    }

    return count;
}