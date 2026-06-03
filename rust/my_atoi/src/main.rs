fn main() {
    let s = String::from("-999999999999999999999999999999999999999");

    println!("{}", my_atoi(s));
}

pub fn my_atoi(s: String) -> i32 {
    let mut s:Vec<char> = s.trim().chars().collect();
    let mut solution = String::new();
    let mut digit_count = 0;

    if s.is_empty(){
        return 0;
    }

    if s[0] == '-'{
        solution.push('-');
        s.remove(0);
    }else if s[0] == '+'{
        s.remove(0);
        solution.push('+');
    }else if s[0] as u8 >= 48 && s[0] as u8 <= 57{
        solution.push('+');
    }

    while s.len() > 0 && s[0] == '0'{
        s.remove(0);
    }

    if s.is_empty() || s[0] == '-' || s[0] == '+'{
        return 0;
    }

    let mut i = 0;

    while i < s.len(){
        if (s[i] as u8) < 48 || (s[i] as u8) > 57{
            break;
        }else{
            solution.push(s[i]);
            digit_count+=1;
        }
        i+=1;
    }

    if digit_count >= 39 && solution.chars().next() == Some('+'){
        return i32::MAX
    }else if digit_count >= 39 && solution.chars().next() == Some('-'){
        return i32::MIN;
    }

    let solution:i128 = if solution.len() < 2{
        0
    }else{
        solution.parse().unwrap()
    };

    if solution > i32::MAX as i128{
        return i32::MAX;
    }else if solution < i32::MIN as i128{
        return i32::MIN;
    }else{
        return solution as i32;
    }
}