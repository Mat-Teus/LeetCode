fn main() {
    let s = String::from("bbba");

    println!("{}", check_string(s));
}

pub fn check_string(s: String) -> bool {  
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < s.len() - 1{
        if chars[i] == 'b' && chars[i + 1] == 'a'{
            return false;
        }

        i+=1;
    }

    return true;
}