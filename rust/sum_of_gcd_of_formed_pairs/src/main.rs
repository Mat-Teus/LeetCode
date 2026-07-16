fn main() {
    let vec = vec![3,6,2,8];

    println!("{}", gcd_sum(vec));
}

pub fn gcd_sum(nums: Vec<i32>) -> i64 {
    let mut max = 0;
    let mut prefixgcd = Vec::new();

    for i in 0..nums.len(){
        if nums[i] > max{
            max = nums[i];
        }

        prefixgcd.push(gcd(nums[i] as i64, max as i64));
    }

    prefixgcd.sort();
    
    let mut ptr_one = 0;
    let mut ptr_two = prefixgcd.len() - 1;
    let mut sum:i64 = 0;

    while ptr_one < ptr_two{
        sum+=gcd(prefixgcd[ptr_one], prefixgcd[ptr_two]);
        ptr_one+=1;
        ptr_two-=1;
    }

    return sum;
}

pub fn gcd(n:i64, n2:i64) -> i64{
    let mut n:i64 = n as i64;
    let mut n2:i64 = n2 as i64;

    while n2 != 0{
        let temp = n2;
        n2 = n%n2;
        n = temp;
    }

    return n;
}