fn main() {
    let n = 1073741825;

    println!("{}", is_power_of_two(n));
}

pub fn is_power_of_two(n: i32) -> bool {
    let mut i = 0;
    let two:i64 = 2;

    loop{
        if two.pow(i) == n as i64{
            return true;
        }else if two.pow(i) > n as i64{
            break;
        }

        i+=1;
    }

    return false;
}