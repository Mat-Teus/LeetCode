use std::collections::HashMap;

fn main() {
    let s = String::from("loonbalxballpoonballoo");
    println!("{}", max_number_of_balloons(s));
}

pub fn max_number_of_balloons(text: String) -> i32 {
    let mut hash = HashMap::new();
    let mut count = 0;

    for i in text.chars(){
        if i == 'b' || i == 'a' || i == 'l' || i == 'o' || i == 'n' {
            *hash.entry(i).or_insert(0) += 1;
        }
    }

    let mut b = *hash.get(&'b').unwrap_or(&0);
    let mut a = *hash.get(&'a').unwrap_or(&0);
    let mut l = *hash.get(&'l').unwrap_or(&0) / 2;
    let mut o = *hash.get(&'o').unwrap_or(&0) / 2;
    let mut n = *hash.get(&'n').unwrap_or(&0);

    while b > 0 && a > 0 && l > 0 && o > 0 && n > 0{
        b-=1;
        a-=1;
        l-=1;
        o-=1;
        n-=1;

        count+=1;
    }

    return count;
}