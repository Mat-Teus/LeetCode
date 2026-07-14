fn main() {
    let vec = vec![6,6,6,6,6,6,6,6,6,19,19,6,6];

    println!("{}",max_distance(vec));
}

pub fn max_distance(colors: Vec<i32>) -> i32 {
    let mut max_d:i32 = 0;

    for house_1 in 0..colors.len(){
        for house_2 in house_1..colors.len(){
            if colors[house_1] != colors[house_2]{
                if house_2 - house_1 > max_d as usize{
                    max_d = (house_2 - house_1) as i32;
                }
            }
        }
    }


    return max_d;
}