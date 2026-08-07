use std::collections::HashMap;

fn main() {
    let strs = vec!["eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()];

    println!("{:?}", group_anagrams(strs));
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut solution = Vec::new();
    let mut hash: HashMap<String, Vec<String>> = HashMap::new();

    for i in strs{
        let mut chars:Vec<char> = i.chars().collect();
        chars.sort();
        let aux:String = chars.into_iter().collect();

        hash.entry(aux).or_default().push(i);
    }

    for (_,j) in hash{
        solution.push(j);
    }

    return solution;        
}