fn main() {
    let vec = vec![0];

    println!("{:?}", sort_array_by_parity(vec));
}

pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    let mut even = Vec::new();
    let mut odd = Vec::new();

    for i in nums{
        if i%2 == 0{
            even.push(i);
        }else{
            odd.push(i);
        }
    }

    even.extend(odd);

    return even;
}