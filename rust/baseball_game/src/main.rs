fn main() {
    let vec = vec!["5".to_string(),"-2".to_string(),"4".to_string(),"C".to_string(),"D".to_string(),"9".to_string(),"+".to_string(),"+".to_string()];
    println!("{}", cal_points(vec));
}

pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut stack:Vec<i32> = Vec::new();
    let mut top:i32 = -1;
    let mut sum = 0;
    let mut i = 0;

    while i < operations.len(){
        if operations[i] == "C"{
            stack.pop();
            top-=1;
        }else if operations[i] == "D"{
            if top > 0{
                stack.push(stack[top as usize] * 2);
                top+=1;
            }
        }else if operations[i] == "+"{
            if top > 1{
                stack.push(stack[top as usize] + stack[top as usize - 1]);
                top+=1;
            }
        }else{
            stack.push(operations[i].parse().unwrap());
            top+=1;
        }

        i+=1;
    }

    for i in stack{
        sum += i;
    }

    return sum;
}