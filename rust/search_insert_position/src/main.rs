fn main() {
    let vec = vec![1,3,5,6];
    let target = 5;

    println!("{}", search_insert(vec, target));
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut middle = nums.len()/2;
    let mut begin = 0;
    let mut end:usize = nums.len();

    while begin < end{
        if nums[middle] == target{
            return middle as i32;
        }else if nums[middle] < target{
            begin = middle + 1;
        }else if nums[middle] > target{
            end = middle;
        }

        middle = (begin + (end - begin)/2) as usize;
    }

    return begin as i32;
}