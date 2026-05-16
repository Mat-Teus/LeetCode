fn main() {
    let x = 8;
    println!("{}", my_sqrt(x));
}

pub fn my_sqrt(x: i32) -> i32 {
    let raiz = (x as f64).powf(0.5);

    return raiz as i32;
}