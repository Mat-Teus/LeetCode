use std::{collections::HashMap};

fn main() {
    let s = String::from("ye");
    let t = String::from("eyr");

    println!("{}", find_the_difference(s, t));
}

pub fn find_the_difference(s: String, t: String) -> char {
    let mut hash = HashMap::new();

    let s:Vec<char> = s.chars().collect();
    let t:Vec<char> = t.chars().collect();

    for i in 0..s.len(){
        *hash.entry(s[i]).or_insert(0) += 1;
    }    

    for j in 0..t.len(){
        *hash.entry(t[j]).or_insert(0) -= 1;

        if hash[&t[j]] < 0{
            return t[j];
        }
    }



    return t[0]
}