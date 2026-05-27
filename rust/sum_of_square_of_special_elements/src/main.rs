fn main() {
    let vec = vec![1,2,3,4];

    println!("{}", sum_of_squares(vec));
}

pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in 0..nums.len(){
        if  nums.len() as i32 % (i+1) as i32 == 0{
            sum = sum + (nums[i] * nums[i]);
        }
    }

    return sum;
}