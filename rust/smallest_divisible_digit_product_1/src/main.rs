fn main() {
    let n = 1;
    let t = 6;

    println!("{}", smallest_number(n, t));
}

pub fn smallest_number(n: i32, t: i32) -> i32 {
    let mut answer = n;
    let mut product = 1;

    loop{
        let mut n = answer;

        while n > 0{
            let digit = n%10;
            n = n/10;
            product*=digit;
        }

        println!("n = {}, product = {}", answer, product);

        if product%t == 0{
            break;
        }else{
            answer+=1;
            product = 1;
        }
    }

    return answer;
}