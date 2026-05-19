fn main() {
    let vec = vec![1, 8];

println!("{}", is_good(vec));
}

pub fn is_good(nums: Vec<i32>) -> bool {
    let mut count = 0;
    let mut vec = nums;
    vec.sort();

    for i in 0..vec.len(){
        if i+1 == vec[i] as usize{
            count+=1;
        }
    }

    if vec.len() == 1{
        return false;
    }else if count == vec.len() - 1 && *vec.last().unwrap() == vec[vec.len() - 2]{
        return true;
    }

   return false;     
}