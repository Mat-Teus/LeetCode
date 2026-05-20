fn main() {
    let n = 6;

    println!("{}", tribonacci(n))
}

pub fn tribonacci(n: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;
    let mut third = 1;

    if n == 0{
        return 0
    }else if n == 1{
        return 1;
    }else if n == 2{
        return 1;
    }

    let mut i = 2;

    while i < n{
        let aux = third + second + first;
        first = second;
        second = third;
        third = aux;
        i+=1;
    }

    return third;
}