fn main() {
    let vec = vec![4,1,2,1,2];

    println!("{}", single_number(vec));
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut solution = 0;

    for i in nums{
        solution^=i;
    }

    return solution;
}