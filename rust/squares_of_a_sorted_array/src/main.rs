fn main() {
    let vec = vec![-4,-1,0,3,10];

    println!("{:?}", sorted_squares(vec));
}

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut solution = Vec::new();

    for i in nums{
        solution.push(i.pow(2));
    }

    solution.sort();

    return solution;
}