fn main() {
    let n = 0;

    println!("{}", sum_and_multiply(n));
}

pub fn sum_and_multiply(n: i32) -> i64 {
    let n = n.to_string();
    let mut aux = String::new();
    let mut sum = 0;

    for i in n.chars(){
        if i != '0'{
            aux.push(i);
            sum+= i.to_digit(10).unwrap() as i64;
        }
    }

    if sum == 0{
        return 0;
    }

    let num:i64 = aux.parse().unwrap();

    return num * sum;
}