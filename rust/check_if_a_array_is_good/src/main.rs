fn main() {
    let vec = vec![1,1];

println!("{}", is_good(vec));
}

pub fn is_good(nums: Vec<i32>) -> bool {
    let max = *nums.iter().max().unwrap();
    let mut count = vec![0; (max + 1) as usize];

    for i in nums.iter() {
        count[*i as usize] += 1;
    }

    let mut i = 1;

    while i < count.len() - 2{
        if count[i] == 0 || count[i] > 1{
            return false;
        }
        i+=1;
    }

    if count[count.len() - 1] == 2{
        return true;
    }else{
        return false;
    }
}