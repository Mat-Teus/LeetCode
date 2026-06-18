fn main() {
    let s = String::from("ab##");
    let t = String::from("c#d#");
    println!("{}", backspace_compare(s, t));
}

pub fn backspace_compare(s: String, t: String) -> bool {
    let mut stack1 = Vec::new();
    let mut stack2 = Vec::new();

    for i in s.chars(){
        if i == '#'{
            stack1.pop();
        }else{
            stack1.push(i);
        }
    }

    for j in t.chars(){
        if j == '#'{
            stack2.pop();
        }else{
            stack2.push(j);
        }
    }

    if stack1 == stack2{
        return true;
    }else{
        return false;
    }
}