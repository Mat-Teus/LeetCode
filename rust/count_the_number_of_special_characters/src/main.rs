fn main() {
    let s = String::from("BBbab");

    println!("{}", number_of_special_chars(s));
}

pub fn number_of_special_chars(word: String) -> i32 {
    let mut chars:Vec<char> = word.chars().collect();
    let mut count = 0;
    let mut i = 0;

    while i < chars.len(){
        let mut j= i + 1;

        while j < chars.len(){
            let is_upper_and_lower = (chars[i] as i32 - chars[j] as i32).abs();
            if is_upper_and_lower == 32{
                count+=1;
                let mut k = 0;
                let letter1 = chars[i];
                let letter2 = chars[j];
                while k < chars.len(){
                    if chars[k] == letter1{
                        chars [k] = '.';
                    }

                    if chars[k] == letter2{
                        chars[k] = ']';
                    }

                    k+=1;
                }
            }
            j+=1;
        }
        i+=1;
    }

    return count;        
}