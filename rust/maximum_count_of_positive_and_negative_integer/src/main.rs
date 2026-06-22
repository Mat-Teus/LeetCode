fn main() {
    let vec = vec![-3,-2,-1,0,0,1,2];

    println!("{}", maximum_count(vec));
}

pub fn maximum_count(nums: Vec<i32>) -> i32 {
    let mut positive = 0;
    let mut negative = 0;

    for i in nums{
        if i > 0{
            positive+=1;
        }else if i < 0{
            negative+=1;
        }
    }

    if positive>=negative{
        return positive;
    }else{
        return negative
    }
}