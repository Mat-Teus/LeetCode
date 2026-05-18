fn main() {
    let mut vec = vec![1,2];
    let k = 7;

    rotate(&mut vec, k);

    println!("{:?}", vec);
}

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let k = k as usize % nums.len();
    
    nums.reverse();
    nums[0..k].reverse();
    nums[k..].reverse();
}