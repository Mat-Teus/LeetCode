fn main() {
    let vec = vec![5,5];

    println!("{}", minimum_cost(vec));
}

pub fn minimum_cost(cost: Vec<i32>) -> i32 {
    let mut vec = cost;
    vec.sort();

    let mut take:i32 = vec.len() as i32 - 1;
    let mut free:i32 = vec.len() as i32 - 3;
    let mut sum = 0;

    for _i in 0..vec.len(){
        if take == free{
            free-=3;
            take-=1;
        }else if take != free{
            sum+=vec[take as usize];
            take-=1;
        }
    }


    return sum;        
}