use std::collections::HashMap;

fn main() {
    let s = String::from("Aabb");

    println!("{}", frequency_sort(s));
}

pub fn frequency_sort(s: String) -> String {
    let mut solution = String::new();
    let mut hash = HashMap::new();

    for i in s.chars(){
        *hash.entry(i).or_insert(0) += 1;
    }

    let mut aux:Vec<(char,i32)> = hash.into_iter().collect();

    aux.sort_by(|a, b| b.1.cmp(&a.1));

    for (i,mut j) in aux{
        while j > 0{
            solution.push(i);
            j-=1;
        }
    }

    return solution;        
}