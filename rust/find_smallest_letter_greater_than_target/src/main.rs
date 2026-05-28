fn main() {
    let letters:Vec<char> = vec!['c','f','j'];
    let target = 'a';

    println!("{}", next_greatest_letter(letters, target));
}

pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let target:u8 = target as u8;

    for &i in &letters{
        if i as u8 > target{
            return i;
        }
    }

    return letters[0];
}