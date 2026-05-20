fn main() {
    let n = 3;
    println!("{}", fib(n));
}

pub fn fib(n: i32) -> i32 {
    let mut prev = 0;
    let mut current = 1;
    let mut i:i32 = 1;

    if n == 0{
        return 0;
    }

    while  i < n{
        if n == 1{
            return 1;
        }else{
            current = current + prev;
            prev = current - prev;
            i+=1;
        }
    }

    return current;
}