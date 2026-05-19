fn main() {
    let vec1 = vec![1,3,2];
    let vec2 = vec![4,3,1,5,2];

    println!("{}", minimum_boxes(vec1, vec2));
}

pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32{
    let mut apple_total:i32 = apple.iter().sum();
    let mut capacity_vec = capacity.clone();

    capacity_vec.sort_by(|a, b| b.cmp(a));

    let mut i = 0;
    let mut boxes = 0;

    while i < capacity_vec.len(){
        apple_total -= capacity_vec[i];
        boxes += 1;
        i+=1;
        
        if apple_total <= 0{
            return boxes;
        }
    }

    return boxes;
}