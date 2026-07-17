fn main() {
    let vec1 = vec![1,1];
    let vec2 = vec![1,1];
    let k = 2;

    println!("{}", mice_and_cheese(vec1, vec2, k));
}

pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
    let mut reward1 = reward1;
    let mut k = k;
    let mut answer = 0;

    for i in 0..reward1.len(){
        reward1[i] = reward1[i] - reward2[i];
    }

    reward1.sort();
    reward1.reverse();

    for i in reward2{
        answer+=i;
    }

    let mut i = 0;

    while k > 0{
        answer+=reward1[i];
        i+=1;
        k-=1;
    }

    return answer;        
}