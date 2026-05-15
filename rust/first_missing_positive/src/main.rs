fn main() {
    let vec = vec![3,4,-1,1,2];

    println!("{}",first_missing_positive(vec));
}

pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut lesser = 1;

    let mut vec_aux: Vec<_> = nums.iter().collect();
    vec_aux.sort();

    for i in 0..nums.len(){
        if vec_aux[i] == &lesser{
            lesser+=1;
        }
    }   
    
    return lesser
}