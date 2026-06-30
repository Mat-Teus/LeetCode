fn main() {
    let mut vec = vec![1,2,3];

    duplicate_zeros(&mut vec);

    println!("{:?}", vec);
}

pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let mut solution = Vec::new();

    for i in 0..arr.len(){
        if arr[i] != 0{
            solution.push(arr[i]);
        }else if arr[i] == 0{
            solution.push(0);
            solution.push(0);
        }
    }

    let error = solution.len() - arr.len();

    arr.clear();
    arr.extend_from_slice(&solution[..solution.len() - error]);
}