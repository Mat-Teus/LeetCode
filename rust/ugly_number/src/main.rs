fn main() {
    let n = -2147483648;

    println!("{}", is_ugly(n));
}

 pub fn is_ugly(n: i32) -> bool {
    let mut n = n;

    if n <= 0{
        return false;
    }

    while n > 1{
        if n % 2 == 0{
            n = n/2;
        }else if n % 3 == 0{
            n = n/3;
        }else if n % 5 == 0{
            n = n/5;
        }else{
            return false;
        }
    }

    return true;        
}