fn main() {
    let vec = vec![7,1,5,3,6,4];

    println!("{}", max_profit(vec));
}

pub fn max_profit(prices: Vec<i32>) -> i32{
    let mut lower = i32::MAX;
    let mut bigger = 0;
    let mut result = 0;

    for i in 0..prices.len(){
        if prices[i] < lower{
            lower = prices[i];
            bigger = lower;
        }

        if prices[i] > bigger{
            bigger = prices[i];
        }

        if bigger - lower > result{
            result = bigger - lower;
        }
    }

    return result;
}