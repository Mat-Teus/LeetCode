fn main() {
    let vec = vec![13,25,83,77];

    println!("{:?}", separate_digits(vec));
}

pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
    let mut solution = Vec::new();
    let mut aux = Vec::new();
    let mut i = 0;

    while i < nums.len(){
        let mut j = nums[i];

        while j > 0{
            aux.push(j%10);
            j = j/10;
        }

        aux.reverse();
        solution.extend(&aux);
        aux.clear();

        i+=1;
    }

    return solution;
}