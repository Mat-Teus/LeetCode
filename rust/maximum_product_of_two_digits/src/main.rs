fn main() {
    let n = 124;

    println!("{}", max_product(n));
}

pub fn max_product(n: i32) -> i32 {
    let mut n = n;
    let mut first = 0;
    let mut second = 0;

    while n > 0{
        let digit = n%10;
        n = n / 10;

        if digit > first{
            second = first;
            first = digit;
        }else if digit > second{
            second = digit
        }
    }

    return first * second;
}