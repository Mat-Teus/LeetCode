fn main() {
    let matrix = vec![
        vec![1,9,3],
        vec![1,2,5],
    ];

    

    println!("{}", maximum_wealth(matrix));
}

pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut bigger = 0;
    let mut sum = 0;
    let mut i = 0;
    let mut j = 0;

    while i < accounts.len(){
        while j < accounts[i].len(){
            sum+=accounts[i][j];
            j+=1;
        }

        if sum > bigger{
            bigger = sum;
        }

        i+=1;
        j=0;
        sum=0;
    }

    return bigger;        
}