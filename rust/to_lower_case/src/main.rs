fn main() {
    let s = String::from("LOVELY");

    println!("{}", to_lower_case(s));
}

pub fn to_lower_case(s: String) -> String {
    let mut answer = String::new();

    for i in s.chars(){
        if i as u8 >= 65 && i as u8 <= 90{
            answer.push((i as u8 + 32) as char);
        }else{
            answer.push(i);
        }
    }

    return answer;
}