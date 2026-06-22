fn main() {
    let s = String::from("42352338");

    println!("{}", largest_good_integer(s));
}

pub fn largest_good_integer(num: String) -> String {
    let mut bigger = -1;
    let chars:Vec<char> = num.chars().collect();

    for i in 0..chars.len() - 2{
        if chars[i] == chars[i+1] && chars[i+1] == chars[i+2]{
            let s:String = chars[i..=i+2].iter().collect();
            let num = s.parse().unwrap();
            if num > bigger{
                bigger = num;
            }
        }
    }

    if bigger == 0{
        return "000".to_string();
    }else if bigger > 0{
        return bigger.to_string();
    }else{
        return "".to_string();
    }
}