fn main() {
    let vec = vec![-2,5,0,2,-2];
    let k = 3;

    println!("{}", largest_sum_after_k_negations(vec, k));
}

pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let mut lowest_index = 0;
    let mut sum = 0;
    let mut k = k;
    let mut i = 0;

    nums.sort();

    while k > 0{
        if nums[i] < nums[lowest_index]{
            lowest_index = i;
        }

        if k > 0 && nums[i] < 0{
            nums[i] *= -1;
            k-=1;
        }else if k > 0 && nums[lowest_index] >= 0{
            while k > 0{
                nums[lowest_index] *= -1;
                k-=1;
            }
        }

        i+=1;

        if i > nums.len() - 1{
            i = 0;
        }
    }

    for i in nums{
        sum+=i;
    }

    return sum;        
}