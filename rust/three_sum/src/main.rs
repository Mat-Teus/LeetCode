fn main() {
    let vec = vec![2,-3,0,-2,-5,-5,-4,1,2,-2,2,0,2,-4,5,5,-10];

    println!("{:?}", three_sum(vec));
}

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums:Vec<i32> = nums;
    nums.sort();
    let mut solution: Vec<Vec<i32>> = Vec::new();
    let mut ptr_still = 0;
    let mut ptr_one = ptr_still as i32 + 1;
    let mut ptr_two:i32 = (nums.len() - 1) as i32;

    while ptr_still < nums.len() - 2{
        while ptr_one < ptr_two{
            let sum = nums[ptr_still] + nums[ptr_one as usize] + nums[ptr_two as usize];

            if sum == 0{
                solution.push(vec![nums[ptr_still], nums[ptr_one as usize], nums[ptr_two as usize]]);
                ptr_one+=1;
                ptr_two-=1;
            }else if sum > 0{
                ptr_two -= 1;
            }else if sum < 0{
                ptr_one+=1;
            }
        }
        ptr_still+=1;
        ptr_one = ptr_still as i32 + 1;
        ptr_two = (nums.len() - 1) as i32;
    }

    solution.sort();
    solution.dedup();   

    return solution;
}