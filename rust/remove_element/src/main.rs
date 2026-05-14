fn main() {
    let mut vec = vec![3,2,2,3];
    let target = 2;

    remove_element(&mut vec, target);

    println!("{:?}", vec);
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k = 0;

    for i in 0..nums.len(){
        if nums[i] == val{
            nums[i] = i32::MAX;
            k += 1;
        }
    }

    nums.sort();

    return nums.len() as i32 - k;
}