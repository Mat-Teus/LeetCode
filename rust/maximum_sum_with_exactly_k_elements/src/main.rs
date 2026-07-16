fn main() {
    let vec = vec![1,2,3,4,5];
    let k = 3;

    println!("{}", maximize_sum(vec, k));
}

pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut greater = 0;

    for i in nums{
        if i > greater{
            greater = i;
        }
    }

    let mut sum = 0;
    let mut k = k;

    while k > 0{
        sum+=greater;
        greater+=1;
        k-=1;
    }

    return sum; 
}