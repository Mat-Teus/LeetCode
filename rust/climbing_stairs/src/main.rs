fn main() {
    let n = 5;

    println!("{}", climb_stairs(n));
}

pub fn climb_stairs(n: i32) -> i32 {
    let mut current = 1;
    let mut previous = 1;
    let mut i = 1;

    if n == 1{
        return 1;
    }

    while i < n{
        let next = current + previous;
        previous = current;
        current = next;

        i+=1;
    }


    return current;
}