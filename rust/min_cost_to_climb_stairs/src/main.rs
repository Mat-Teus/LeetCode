use std::cmp::min;

fn main() {
    let vec = vec![1,100,1,1,1,100,1,1,100,1];

    println!("{}", min_cost_climbing_stairs(vec));
}

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut dp = Vec::new();
    dp.push(cost[0]);
    dp.push(cost[1]);

    for i in 2..cost.len(){
        dp.push(min(dp[i - 1], dp[i - 2]) + cost[i]);
    }

    return min(dp[dp.len() - 1], dp[dp.len() - 2]);        
}