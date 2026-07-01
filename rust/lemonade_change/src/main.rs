fn main() {
    let vec = vec![5,5,5,10,20];

    println!("{}", lemonade_change(vec));
}

pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut count_5:i32 = 0;
    let mut count_10:i32 = 0;

    for i in 0..bills.len(){
        if bills[i] == 5{
            count_5+=1;
        }else if bills[i] == 10{
            count_10+=1;
            count_5-=1;
        }else if bills[i] == 20 && count_10 <= 0{
            count_5-=3;
        }else if bills[i] == 20 && count_10 > 0{
            count_10-=1;
            count_5-=1;
        }

        if count_10 < 0 || count_5 < 0{
            return false;
        }
    }

    return true;
}