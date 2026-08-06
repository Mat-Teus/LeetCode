use std::collections::HashMap;

fn main() {
    let hand = vec![1,2,3,6,2,3,4,7,8];
    let group_size = 3;

    println!("{}", is_n_straight_hand(hand, group_size));
}

pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
    if hand.len() as i32 % group_size != 0{
        return false;
    }

    let mut hash = HashMap::new();
    let mut hand = hand;
    hand.sort();

    for i in 0..hand.len(){
        *hash.entry(hand[i]).or_insert(0) += 1;
    }

    let mut k = 0;

    while !hash.is_empty(){
        let mut num = *hash.keys().min().unwrap();

        while k < group_size as usize{
            if hash.contains_key(&num){
                let mut remove = false;
            

            if let Some(freq) = hash.get_mut(&num){
                *freq -= 1;
                remove = *freq == 0;
            }

            if remove{
                hash.remove(&num);
            }

                num += 1;
                k+=1;
            }else{
                return false;
            }
        }
        k = 0;
    }

    return true;        
}