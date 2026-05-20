fn main() {
    let vec = vec![0,1,2,3];

    println!("{}", find_min(vec));
}

pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut begin:usize = 0;
    let mut end = nums.len() - 1;

    while begin < end{
    if nums[begin] <= nums[end]{
        break;
    }
    let middle = begin + (end - begin)/2;

    if nums[middle] > nums[end]{
        begin = middle + 1;
    }else{
        end = middle;
        }

    }

    return nums[begin];
}