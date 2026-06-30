fn main() {
    let num = String::from("123");
    let k = 1;

    println!("{}", remove_kdigits(num, k));
}

pub fn remove_kdigits(num: String, k: i32) -> String {
    let mut stack = Vec::new();
    let mut k = k;
    let num:Vec<i32> = num.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    if k == num.len() as i32{
        return "0".to_string();
    }

    for i in num {
        while k > 0 && !stack.is_empty() && stack.last().unwrap() > &i {
            stack.pop();
            k -= 1;
        }

        stack.push(i);
    }

    while k > 0{
        stack.pop();
        k-=1;
    }

    let mut result:String = stack.iter().map(|d| d.to_string()).collect();
    result = result.trim_start_matches('0').to_string();

    if result == ""{
        return "0".to_string()
    }else{
        return result;
    }
}