fn main() {
    let vec = vec![999,19,199];

    println!("{}", min_element(vec));
}

pub fn min_element(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut minimum = i32::MAX;

    for i in 0..nums.len(){
        let num = nums[i];
        let digits:Vec<i32> = num.to_string().chars().map(|d| d.to_digit(10).unwrap() as i32).collect();
        for j in 0..digits.len(){
            sum+=digits[j];
        }

        if sum < minimum{
            minimum = sum;
        }

        sum = 0;
    }

    return minimum;        
}