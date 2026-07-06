fn main() {
    let n = 21376;

    println!("{}", divisor_game(n));
}

pub fn divisor_game(n: i32) -> bool {
    if n % 2 == 0{
        return true;
    }else{
        return false;
    }
}