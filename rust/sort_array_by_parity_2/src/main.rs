fn main() {
    let vec = vec![1,2,4,3];

    println!("{:?}", sort_array_by_parity_ii(vec));
}

pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut ptr_one = 0;
    let mut ptr_two = 0;

    while nums[ptr_one] % 2 != 0{
        ptr_two += 1;

        if nums[ptr_two] % 2 == 0{
            nums.swap(ptr_one, ptr_two);
        }
    }

    ptr_one = 0;
    ptr_two = 0;

    while ptr_one != nums.len() - 1{
        if ptr_two < nums.len() && ptr_one == ptr_two{
            ptr_two+=1;
        }

        if nums[ptr_one] % 2 == 0 && nums[ptr_two] % 2 != 0{
            ptr_one+=1;
        }else if nums[ptr_one] % 2 != 0 && nums[ptr_two] % 2 == 0{
            ptr_one+=1;
        }else if nums[ptr_one] % 2 != 0 && nums[ptr_two] % 2 != 0{
            ptr_one+=1;
            while nums[ptr_two] % 2 != 0{
                ptr_two+=1;
            }

            nums.swap(ptr_one, ptr_two);
            ptr_two = ptr_one;
        }else if nums[ptr_one] % 2 == 0 && nums[ptr_two] % 2 == 0{
            ptr_one+=1;
            while nums[ptr_two] % 2 == 0{
                ptr_two+=1;
            }

            nums.swap(ptr_one, ptr_two);
            ptr_two = ptr_one;
        }
    }

    return nums;
}