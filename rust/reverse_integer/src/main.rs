fn main() {
    let n = -2147483648;

    println!("{}", reverse(n));
}

pub fn reverse(x: i32) -> i32 {
    let n = x.to_string();
    let mut n: Vec<char> = n.chars().collect();
    let mut is_negative = false;

    if n[0] == '-'{
        is_negative = true;
        n.remove(0);
    }

    let mut solution = String::new();

    let mut i = 0;

    while i < n.len(){
        solution.push(n[i]);
        i+=1;
    }

    let solution:String = solution.chars().rev().collect();
    let mut solution:i128 = solution.parse().unwrap();

    if is_negative == true{
        if solution > i32::MAX as i128{
            return 0;
        }else{
            solution*=-1;
            return solution as i32;
        }
    }

    if solution > i32::MAX as i128{
        return 0;
    }else{
        return solution as i32;
    }
}