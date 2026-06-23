use std::collections::HashSet;

fn main() {
    let n = 2;

    println!("{}", is_happy(n));
}

pub fn is_happy(n: i32) -> bool {
    let mut hash = HashSet::new();
    let mut n = n;

    while n != 1{
        let mut aux = 0;
        hash.insert(n);

        while n > 0{
            aux += (n%10).pow(2);
            n /= 10;
        }

        n = aux;
        if hash.contains(&n){
            return false;
        }
    }

    return true;        
}