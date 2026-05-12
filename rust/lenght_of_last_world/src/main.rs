fn main() {
    let s = String::from(" ");

    println!("{}",length_of_last_word(s));
}

pub fn length_of_last_word(s: String) -> i32 {
    let mut lenght = 0;

    let s = s.split_whitespace().collect::<Vec<&str>>().join(" ");

    for i in s.chars(){
        if i == ' '{
            lenght = 0;
        }else{
            lenght += 1;
        }
    }

    return lenght;
}