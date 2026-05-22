fn main() {
    let s = String::from("0P");

    println!("{}", is_palindrome(s));
}

pub fn is_palindrome(s: String) -> bool {
    let s: Vec<char> = s.chars().filter(|c| c.is_alphanumeric()).flat_map(|c| c.to_lowercase()).collect();

    if s.is_empty(){
        return true;
    }

    let mut ptr_one = 0;
    let mut ptr_two:usize= s.len() - 1;
    
    while ptr_one != ptr_two{
        if s[ptr_one] != s[ptr_two]{
            return false;
        }
        ptr_one+=1;
        ptr_two-=1;
    }

    return true;       
}