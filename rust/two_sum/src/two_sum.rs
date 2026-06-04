fn main() {
    let nums = vec![3,3];
    let target: i32 = 6;

    println!("{:?}", two_sum(nums, target));
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash = HashMap::new();
    let mut solution = Vec::new();

    for i in 0..nums.len(){
        let complement = target - nums[i];

        if hash.contains_key(&complement){
            solution.push(*hash.get(&complement).unwrap() as i32);
            solution.push(i as i32);
            return solution;
        }

            
        hash.insert(nums[i], i);
    }

    return solution;
}