use std::cmp::max;

fn main() {
    let vec = vec![3,3,5,0,0,3,1,4];

    println!("{}", max_profit(vec));
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let prices = prices;
    let mut purchase = i32::MIN;
    let mut sell = 0;
    let mut purchase_max = i32::MIN;
    let mut sell_max = 0;

    for i in 0..prices.len(){
        purchase = max(purchase, - prices[i]);
        sell = max(sell, purchase + prices[i]);

        purchase_max = max(purchase_max,  sell - prices[i]);
        sell_max = max(sell_max, purchase_max + prices[i]);
    }

    return sell_max;
}