fn main() {
    let vec = vec![7,8,9];
    let target = -1;

    println!("{}", three_sum_closest(vec, target));
}

pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort();

    let mut ptr_still = 0;
    let mut ptr_one:i32 = ptr_still + 1;
    let mut ptr_two:i32 = (nums.len() - 1) as i32;
    let mut closest = nums[0] + nums[1] + nums[2];

    while ptr_still < (nums.len() - 2) as i32{
        while ptr_one < ptr_two{
            let aux = nums[ptr_still as usize] + nums[ptr_one as usize] + nums[ptr_two as usize];
    
            if (nums[ptr_still as usize] + nums[ptr_one as usize] + nums[ptr_two as usize] - target).abs() < (closest - target).abs(){
                closest = aux;
            }
            
            if aux < target{
                ptr_one+=1;
            }else if aux > target{
                ptr_two-=1;
            }else{
                return target;
            }
        }
        ptr_still+=1;
        ptr_one = ptr_still + 1;
        ptr_two = (nums.len() - 1) as i32;
    }


    return closest;
}