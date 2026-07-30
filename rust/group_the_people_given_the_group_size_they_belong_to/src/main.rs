use std::collections::HashMap;

fn main() {
    let vec = vec![3,3,3,3,3,1,3];

    println!("{:?}", group_the_people(vec));
}

pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
    let mut solution: Vec<Vec<i32>> = Vec::new();
    let mut hash = HashMap::new();

    for i in 0..group_sizes.len(){
        hash.insert(i, group_sizes[i]);
    }

    let mut values:Vec<(&usize, &i32)> = hash.iter().collect();

    values.sort_by(|a, b| b.1.cmp(a.1));

    let mut aux = vec![];

    for (i,j) in values{
        if aux.len() < *j as usize{
            aux.push(*i as i32);
        }else{
            solution.push(std::mem::take(&mut aux));
            aux.clear();
            aux.push(*i as i32);
        }
    }

    if !aux.is_empty(){
        solution.push(aux);
    }

    return solution;
}