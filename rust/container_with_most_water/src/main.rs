use std::cmp::min;

fn main() {
    let vec = vec![1,8,6,2,5,4,8,3,7];

    println!("{}", max_area(vec));
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut ptr_one = 0;
    let mut ptr_two = height.len() - 1;
    let mut max_height = 0;

    while ptr_one < ptr_two{
        let min = min(height[ptr_one], height[ptr_two]);
        let aux_height = (ptr_two - ptr_one) as i32 * min;

        if aux_height > max_height{
            max_height = aux_height;
        }

        if height[ptr_one] > height[ptr_two]{
            ptr_two-=1;
        }else if height[ptr_two] > height[ptr_one]{
            ptr_one+=1;
        }else{
            ptr_one+=1;
            ptr_two-=1;
        }
    }

    return max_height;        
}