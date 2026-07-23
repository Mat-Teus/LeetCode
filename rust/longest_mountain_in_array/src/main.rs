fn main() {
    let vec = vec![0,2,0,2,1,2,3,4,4,1];

    println!("{}", longest_mountain(vec));
}

pub fn longest_mountain(arr: Vec<i32>) -> i32 {
    let mut ptr_one = 0;
    let mut climbing = false;
    let mut descending = false;
    let mut longest_distance = 0;
    let mut distance = 0;

    for i in 1..arr.len(){
        if arr[i] == arr[i - 1]{
            distance = 0;
            descending = false;
            climbing = false;
        }

        if arr[i] > arr[i-1] && climbing == false{
            distance = 0;
            climbing = true;
            descending = false;
            ptr_one = i - 1;
        }

        if climbing == true && arr[i] < arr[i - 1]{
            descending = true;
            climbing = false;
            distance = 0;
        }

        if descending == true{
            distance = i - ptr_one + 1;
        }

        if distance > longest_distance{
            longest_distance = distance;
        }
    }

    return longest_distance as i32;     
}