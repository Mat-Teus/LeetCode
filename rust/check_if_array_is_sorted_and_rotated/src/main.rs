fn main() {
    let vec = vec![2,1,3,4];

    println!("{}", check(vec));
}

pub fn check(nums: Vec<i32>) -> bool {
    let mut count = 0;
    let mut i = 0;

    while i < nums.len(){
        if nums[i] > nums[(i + 1) % nums.len()]{
            count+=1;
        }

        i+=1;
    }

    if count <= 1{
        return true;
    }else{
        return false;
    }
}