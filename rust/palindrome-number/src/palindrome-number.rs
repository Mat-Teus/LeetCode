fn main() {
    println!("{}", is_palindrome(100));
}

pub fn is_palindrome(x: i32) -> bool {
    let chars: Vec<char> = x.to_string().chars().collect();
    let mut begin = 0;
    let mut end = chars.len() - 1;

    while begin < end{
        let ptr_one = &chars[begin];
        let prt_two = &chars[end];

        if ptr_one != prt_two{
            return false;
        }

        begin += 1;
        end -= 1;
    }

    return true;
}