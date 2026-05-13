use std::collections::HashSet;

fn main() {
    let s = String::from("insertstring");

    println!("{}",length_of_longest_substring(s));
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut longest_substring = 0;
    let mut hash = HashSet::new();

    for right in 0..chars.len(){
        while hash.contains(&chars[right]){
            hash.remove(&chars[left]);
            left += 1;
        }   

        hash.insert(&chars[right]);

        if right - left + 1 > longest_substring{
            longest_substring = right - left +1;
        }
    }

    return longest_substring as i32;    
}