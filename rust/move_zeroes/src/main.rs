fn main() {
    let mut vec = vec![0,1,0,3,12];
    move_zeroes(&mut vec);
    println!("{:?}", vec);
}

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut ptr_write = 0;
    let mut ptr_read = 0;
    let mut count_zero = 0;

    while ptr_write < nums.len(){
        if nums[ptr_write] == 0{
            count_zero+=1;
        }

        if nums[ptr_write] != 0{
            nums[ptr_read] = nums[ptr_write];
            ptr_read+=1;
        }

        ptr_write+=1;
    }

    let mut i = nums.len() - (count_zero);

    while i < nums.len(){
        nums[i] = 0;
        i+=1;
    }
}