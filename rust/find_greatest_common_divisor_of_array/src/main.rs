fn main() {
    let vec = vec![2,5,6,9,10];

    println!("{}", find_gcd(vec));
}

pub fn find_gcd(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut min = 1001;     

    for i in nums{
        if i > max{
            max = i;
        }
        
        if i < min{
            min = i;
        }
    }

    return gcd(min, max);
}

pub fn gcd(n1:i32, n2:i32) -> i32{
    let mut n1 = n1;
    let mut n2 = n2;

    while n2 != 0{
        let temp = n2;
        n2 = n1%n2;
        n1 = temp;
    }

    return n1;
}