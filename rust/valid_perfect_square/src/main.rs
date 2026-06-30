fn main() {
    let num = 1;

    println!("{}", is_perfect_square(num));
}

pub fn is_perfect_square(num: i32) -> bool {
    if num == 1{
        return true;
    }

    let mut i = 1;

    while i <= num/2{
        if i * i == num{
            return true;
        }

        i+=1;
    }

    return false;
}