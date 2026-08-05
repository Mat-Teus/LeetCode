fn main() {
    let vec = vec![1,2,3,4,5];
    let cost = vec![3,4,5,1,2];

    println!("{}", can_complete_circuit(vec, cost));
}

pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut aux = Vec::new();

    for i in 0..gas.len(){
        aux.push(gas[i] - cost[i]);
    }

    let mut sum = 0;

    if aux.iter().sum::<i32>() < 0 {
        return -1;
    }

    let mut answer = 0;

    for i in 0..aux.len(){
        sum += aux[i];

        if sum < 0{
            sum = 0;
            answer = i + 1;
        }
    }

    return answer as i32;        
}