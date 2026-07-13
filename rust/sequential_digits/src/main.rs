fn main() {
    let low = 1000;
    let high = 13000;

    println!("{:?}", sequential_digits(low, high));
}

pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    let mut solution = Vec::new();
    let mut start = 1;
    let mut number = start;
    let mut next = start;

    while start <= 9{

        while next < 9{
            next+=1;
            number = number * 10 + next;
            if number >= low && number <= high{
                solution.push(number);
            } 
        }
        
        start+=1;
        number = start;
        next = start;
    }

    solution.sort();

    return solution;        
}