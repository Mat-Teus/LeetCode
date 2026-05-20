use std::vec;

fn main() {
    let vec1 = vec![1,2,3];
    let vec2 = vec![1,2,3];

    println!("{:?}", find_the_prefix_common_array(vec1, vec2));
}

pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut vec_solution:Vec<i32> = Vec::new();
    let mut vec_bool:Vec<bool> = vec![false; a.len() + 1];
    let mut count = 0;

    for i in 0..a.len(){
        if vec_bool[a[i] as usize]{
            count += 1;
        }else{
            vec_bool[a[i] as usize] = true;
        }

        if vec_bool[b[i] as usize]{
            count += 1;
        }else{
            vec_bool[b[i] as usize] = true;
        }

        vec_solution.push(count);
    }

    return vec_solution;
}