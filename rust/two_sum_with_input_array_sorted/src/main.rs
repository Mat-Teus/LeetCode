fn main() {
    let vec = vec![2,7,11,15];
    let target = 9;

    println!("{:?}", two_sum(vec, target));
}

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut solution = Vec::new();
    let mut ptr_one = 0;
    let mut ptr_two = numbers.len() - 1;

    while ptr_one < ptr_two{
        let sum = numbers[ptr_one] + numbers[ptr_two];
        if sum > target{
            ptr_two-=1;
        }else if sum < target{
            ptr_one+=1;
        }else if sum == target{
            solution.push(ptr_one as i32 + 1);
            solution.push(ptr_two as i32 + 1);
            break;
        }
    }

    return solution;
}