fn main() {
    let vec = vec![5];
    let target = 5;

    println!("{}", search(vec, target));
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut begin = 0;
    let mut end:i32 = nums.len() as i32 - 1;

    while begin <= end{
        let middle = begin + (end - begin)/2;

        if nums[middle as usize] == target{
            return middle;
        }else if nums[middle as usize] > target{
            end = middle - 1;
        }else if nums[middle as usize] < target{
            begin = middle + 1;
        }
    }

    return -1;        
}