fn main() {
    let s = String::from("anagram");
    let t = String::from("negaram");

    println!("{}", is_anagram(s, t));
}

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len(){
        return false;
    }

    let mut s:Vec<char> = s.chars().collect();
    let mut t:Vec<char> = t.chars().collect();

    s.sort();
    t.sort();

    for i in 0..s.len(){
        if s[i] != t[i]{
            return false;
        }
    }

    return true;
}