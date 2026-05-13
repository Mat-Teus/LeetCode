fn main() {
    let s = String::from("()[]{}");

    println!("{}", is_valid(s));
}

pub fn is_valid(s: String) -> bool{
    let chars: Vec<char> = s.chars().collect();
    let mut stack: Vec<char> = Vec::new();

    for &i in &chars {
        if i == '(' || i == '[' || i == '{' {
            stack.push(i);
        }else if i == ')'{
            if stack.last() == Some(&'('){
                stack.pop();
            }else{
                return false;
            }
        }
        else if i == ']'{
            if stack.last() == Some(&'['){
                stack.pop();
            }else{
                return false;
            }
        }
        else if i == '}'{
            if stack.last() == Some(&'{'){
                stack.pop();
            }else{
                return false;
            }
        }
    }


    if stack.is_empty(){
        return true;
    }else{
        return false;
    }
}