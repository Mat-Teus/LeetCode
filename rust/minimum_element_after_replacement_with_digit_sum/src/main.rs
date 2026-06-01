fn main() {
    let vec = vec![999,19,199];

    println!("{}", min_element(vec));
}

pub fn min_element(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut minimum = i32::MAX;
    let mut i = 0;

    while i < nums.len(){
        let mut j = nums[i];

        while j > 0{
            sum+=j%10;
            j = j/10;
        }

        if minimum > sum{
            minimum = sum;
        }

        i+=1;
        sum = 0;
    }

    return minimum;        
}