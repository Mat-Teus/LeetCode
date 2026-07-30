use std::collections::HashMap;

fn main() {
    let s = String::from("amrvxnhsewkoipjyuclgtdbfq");

    println!("{}", minimum_pushes(s));
}

pub fn minimum_pushes(word: String) -> i32 {
    let mut hash = HashMap::new();
    let mut count = 0;


    for i in word.chars(){
        *hash.entry(i).or_insert(0) += 1;
    }

    let mut freq:Vec<i32> = hash.values().copied().collect();

    freq.sort();
    freq.reverse();

    let mut i = 0;

    while i < freq.len(){
        if i < 8{
            while freq[i] > 0{
                freq[i]-=1;
                count+=1;
            }
        }else if i >= 8 && i < 16{
            while freq[i] > 0{
                freq[i]-=1;
                count+=2;
            }
        }else if i >= 16 && i < 24{
            while freq[i] > 0{
                freq[i]-=1;
                count+=3;
            }
        }else if i >= 24{
            while freq[i] > 0{
                freq[i]-=1;
                count+=4;
            }
        }

        i+=1;
    }

    return count;        
}