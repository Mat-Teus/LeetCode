fn main() {
    let n = 25;

    println!("{}", mirror_distance(n));
}

pub fn mirror_distance(n: i32) -> i32 {
    let reversed:String = n.to_string().chars().rev().collect();
    let reversed_n:i32 = reversed.parse().unwrap();

    return (n-reversed_n).abs();
}