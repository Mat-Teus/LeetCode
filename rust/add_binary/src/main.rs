fn main() {
    let a = String::from("1010");
    let b = String::from("1011");

    println!("{}", add_binary(a, b))
}

pub fn add_binary(a: String, b: String) -> String {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut carry = 0;
    let mut ptr_one = a.len() as i32 - 1;
    let mut ptr_two = b.len() as i32 - 1;
    let mut solution = String::new();

    while ptr_one >= 0 || ptr_two >= 0 || carry > 0{
        let mut sum = carry;

        if ptr_one >= 0{
            sum += (a[ptr_one as usize] as u8 - b'0') as i32;
            ptr_one -= 1;
        }

        if ptr_two >= 0{
            sum += (b[ptr_two as usize] as u8 - b'0') as i32;
            ptr_two -= 1;
        }

        let bit = sum % 2;
        carry = sum/2;

        solution.push(if bit == 0 {'0'} else {'1'});
    }

    let solution = solution.chars().rev().collect();

    return solution
}