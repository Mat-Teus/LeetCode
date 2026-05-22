fn main() {
    let vec = vec![7,8,9];
    let target = -1;

    println!("{}", three_sum_closest(vec, target));
}

pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort();

    let mut ptr_still = 0;
    let mut ptr_one:i32 = 0;
    let mut ptr_two:i32 = (nums.len() - 1) as i32;
    let mut closest = nums[0] + nums[1] + nums[2];

    while ptr_still < nums.len(){
        while ptr_one < ptr_two{
            if ptr_one == ptr_still as i32{
                ptr_one+=1;
            }

            if ptr_two == ptr_still as i32{
                ptr_two -= 1;
            }

            if ptr_one == ptr_two{
                break;
            }

            if ptr_one >= nums.len() as i32{
                break;
            }

            if ptr_two < 0{
                break;
            }

            let aux = nums[ptr_still] + nums[ptr_one as usize] + nums[ptr_two as usize];
            if aux == target{
                closest = aux;
                return closest;
            }else if (aux - target).abs() <= (closest - target).abs(){
                closest = aux;
            }
            
            if aux < target{
                ptr_one+=1;
            }else if aux > target{
                ptr_two-=1;
            }
        }
        ptr_still+=1;
        ptr_one = 0;
        ptr_two = (nums.len() - 1) as i32;
    }


    return closest;
}