use std::collections::HashMap;

fn main() {
    let low = 1;
    let high = 10;

    println!("{}", count_balls(low, high));
}

pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
    let mut hash = HashMap::new();
    let mut bigger = -1;

    for i in low_limit..=high_limit{
        let mut n = i;
        let mut box_ = 0;

        while n > 0{
            let digit = n%10;
            box_+=digit;
            n/=10;
        }
        *hash.entry(box_).or_insert(0) += 1;
    }

    for (_j, k) in hash{
        if k > bigger{
            bigger = k;
        }
    }

    return bigger;
}