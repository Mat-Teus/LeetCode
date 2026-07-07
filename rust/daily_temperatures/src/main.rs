fn main() {
    let vec = vec![73,74,75,71,69,72,76,73];

    println!("{:?}", daily_temperatures(vec));
}

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut solution = vec![0;temperatures.len()];
    let mut stack = Vec::new();

    for i in 0..temperatures.len(){
        while !stack.is_empty() && temperatures[stack[stack.len() - 1]] < temperatures[i]{
            let aux = stack[stack.len() - 1];
            solution[stack[stack.len() - 1]] = (i - aux) as i32;
            stack.pop();
        }

        stack.push(i);
    }

    return solution;
}