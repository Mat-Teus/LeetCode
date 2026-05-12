fn main() {
    let mut vec= Vec::new();
    vec.push(String::from("Flower"));
    vec.push(String::from("Flight"));
    vec.push(String::from("Fliyer"));

    println!("{}", longest_common_prefix(vec));
}


pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut common_prefix = String::new();
    let lesser_string = strs.iter().map(|s| s.chars().count()).min().unwrap();
    let mut same = true;

    for i in 0..lesser_string{
        let c1 = strs[0].chars().nth(i).unwrap();
        
        for j in &strs{
            if j.chars().nth(i).unwrap() != c1{
                same = false
            }
        }

        if same == true{
            common_prefix.push(c1);
        }else{
            break;
        }
    }

    return common_prefix;
}