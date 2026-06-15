fn main() {
    let g = vec![1,2,3];
    let s = vec![1, 2];
    println!("{}", find_content_children(g, s));
}

pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
    let mut g = g;
    let mut s = s;
    let mut ptr_one = 0;
    let mut ptr_two = 0;
    let mut count = 0;

    g.sort();
    s.sort();

    while ptr_two < s.len() && ptr_one < g.len(){
        if g[ptr_one] <= s[ptr_two]{
            count+=1;
            ptr_two+=1;
            ptr_one+=1;

        }else{
            ptr_two+=1;
        }
    }

    return count;        
}