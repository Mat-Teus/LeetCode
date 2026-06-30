fn main() {
    let vec = vec![3,7];

    println!("{}", max_product(vec));
}

pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut first = 0;
    let mut second = -1;

    for i in 0..nums.len(){
        if nums[i]>first{
            second = first;
            first = nums[i];
        }else if nums[i]>second{
            second = nums[i];
        }
    }

    return (first - 1) * (second - 1);
}