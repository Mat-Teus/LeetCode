use std::collections::HashMap;

fn main() {
    let s = String::from("papar");
    let t = String::from("title");

    println!("{}", is_isomorphic(s, t));
}

pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut hash = HashMap::new();

    let s:Vec<char> = s.chars().collect();
    let t:Vec<char> = t.chars().collect();

    if s.len() != t.len(){
        return false;
    }

    for i in 0..s.len(){
        if !hash.contains_key(&s[i]){
            hash.insert(s[i], t[i]);
        }else{
            if hash.get(&s[i]) != Some(&t[i]){
                return false;
            }
        }
    }

    hash.clear();

    for j in 0..t.len(){
        if !hash.contains_key(&t[j]){
            hash.insert(t[j], s[j]);
        }else{
            if hash.get(&t[j]) != Some(&s[j]){
                return false;
            }
        }
    }

    return true;
}