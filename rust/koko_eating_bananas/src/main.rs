fn main() {
    let vec = vec![30,11,23,4,20];
    let h = 5;

    println!("{}", min_eating_speed(vec, h));
}

pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut right = 0;
    let mut left = 1;

    for i in 0..piles.len(){
        if piles[i] > right{
            right = piles[i] as i32;
        }
    }

    while left < right{
        let mut hours = 0;
        let middle = left + (right - left)/2;

        for i in &piles{
            hours+= (i + middle - 1)/middle;
        }

        if hours <= h{
            right = middle;
        }else{
            left = middle + 1;
        }

    }
    

    return left;
}