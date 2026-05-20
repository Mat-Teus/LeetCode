fn main() {
    let sequence = String::from("aaabaaaabaaabaaaabaaaabaaaabaaaaba");
    let word = String::from("aaaba");

    println!("{}", max_repeating(sequence, word));
}

pub fn max_repeating(sequence: String, word: String) -> i32 {
    let mut count = 0;
    let mut aux = word.clone();

    while sequence.contains(&aux){
        count+=1;
        aux += &word;
    }

    return count;
}