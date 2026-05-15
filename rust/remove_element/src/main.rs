fn main() {
    let mut vec = vec![3,2,2,3, 3];
    let target = 2;

    remove_element(&mut vec, target);
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut write = 0;

    for read in 0..nums.len(){
        if nums[read] != val{
            nums[write] = nums[read];
            write += 1;
        }
    }

    return write as i32;
}