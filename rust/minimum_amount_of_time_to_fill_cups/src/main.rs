fn main() {
    let vec = vec![5,4,4];

    println!("{}", fill_cups(vec));
}

pub fn fill_cups(amount: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut amount = amount;

    amount.sort();

    while amount[2] > 0{
        amount[2] -= 1;
        amount[1] -= 1;
        count+=1;

        amount.sort();
    }

    return count;        
}