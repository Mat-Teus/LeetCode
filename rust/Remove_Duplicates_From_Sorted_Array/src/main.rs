fn main() {
    let mut vec = vec![1,1,1];
    remove_duplicates(&mut vec);

    println!("{:?}", vec);
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut ptr_one = 0;

    for i in 1..nums.len(){
        if nums[i] != nums[ptr_one]{
            ptr_one += 1;
            nums[ptr_one] = nums[i];
        }
    }

    return (ptr_one + 1) as i32;
}