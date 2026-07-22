fn main() {
    let vec = vec![3,3];

    println!("{}", max_profit(vec));
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut ptr_one = 0;
    let mut ptr_two = 1;
    let mut result = 0;

    while ptr_one < prices.len() && ptr_two < prices.len(){
        if prices[ptr_one] >= prices[ptr_two]{
            ptr_one += 1;
            ptr_two += 1;
        }else if prices[ptr_one] < prices[ptr_two]{
            if ptr_two + 1 >= prices.len(){
                result+=prices[ptr_two] - prices[ptr_one];
                break;
            }
            if prices[ptr_two] > prices[ptr_two + 1]{
                result+=prices[ptr_two] - prices[ptr_one];
                ptr_one = ptr_two + 1;
                ptr_two += 2;
            }else{
                ptr_two+=1;
            }
        }else if ptr_one == ptr_two{
            ptr_one+=1;
            ptr_two+=2;
        }else if ptr_two >= prices.len(){
            break;
        }
    }

    return result;        
}