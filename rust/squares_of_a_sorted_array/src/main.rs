fn main() {
    let vec = vec![1,2,3,4,5];

    println!("{:?}", sorted_squares(vec));
}

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut solution = vec![0;nums.len()];
    let mut ptr_one = 0;
    let mut ptr_two:usize = nums.len() - 1;
    let mut pos = nums.len() - 1;

    while pos > 0{
        if nums[ptr_one].abs() > nums[ptr_two].abs(){
            solution[pos] = nums[ptr_one].pow(2);
            ptr_one+=1;
        }else{
            solution[pos] = nums[ptr_two].pow(2);
            ptr_two-=1;
        }

        pos -= 1;
    }

    solution[0] = nums[ptr_one].pow(2);

    return solution;
}