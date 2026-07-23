fn main() {
    let mut vec = vec![0,0,1,1,1,1,2,3,3];

    println!("{}", remove_duplicates(&mut vec));
    println!("{:?}", vec);
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut ptr_two = 2;

    if nums.len() <= 2{
        return nums.len() as i32;
    }

    for i in 2..nums.len(){
        if nums[i] != nums[ptr_two - 2]{
            nums[ptr_two] = nums[i];
            ptr_two+=1;
        }
    }

    return ptr_two as i32;
}