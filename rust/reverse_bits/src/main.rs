fn main() {
    let n = 1;

    println!("{}", reverse_bits(n));
}

pub fn reverse_bits(n: i32) -> i32 {
    return n.reverse_bits()
}