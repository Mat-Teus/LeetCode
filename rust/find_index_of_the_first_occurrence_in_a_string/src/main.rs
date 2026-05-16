fn main() {
    let haystack = String::from("mississippi");
    let needle = String::from("issip");

    println!("{}",str_str(haystack, needle));
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    return haystack.find(&needle).map(|x| x as i32).unwrap_or(-1);
}