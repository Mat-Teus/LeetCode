fn main() {
    let vec = vec![9,12,5,10,14,3,10];
    let pivot = 10;

    println!("{:?}",pivot_array(vec, pivot));
}

pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut vec:Vec<i32> = Vec::new();
    let mut right:Vec<i32> = Vec::new();
    let mut left:Vec<i32> = Vec::new();

    for i in nums{
        if i < pivot{
            left.push(i);
        }else if i > pivot{
            right.push(i);
        }else if i == pivot{
            vec.push(i);
        }
    }

    left.extend(vec);
    left.extend(right);

    return left;
}