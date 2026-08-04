fn main() {
    let vec = vec![10,6,8,7,7,8];
    let coins = 5;

    println!("{}", max_ice_cream(vec, coins));
}

pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
    let mut coins = coins;
    let max_cost = *costs.iter().max().unwrap() as usize;
    let mut occurences: Vec<i32> = vec![0; max_cost + 1];
    let mut answer = 0;

    for &data in costs.iter() {
        occurences[data as usize] += 1;
    }

    for i in 0..occurences.len(){
        for _j in 0..occurences[i]{
            if coins >= i as i32 {
                coins -= i as i32;
                answer += 1;
            }else if coins < 0{
                break;
            }
        }
    }

    return answer;        
}