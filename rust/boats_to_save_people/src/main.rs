fn main() {
    let vec = vec![1];
    let n = 3;

    println!("{}", num_rescue_boats(vec, n));
}

pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
    let mut ptr_one = 0;
    let mut ptr_two = people.len() - 1;
    let mut people = people;
    let mut boats = 0;

    people.sort();

    while ptr_one < ptr_two{
        let sum = people[ptr_one] + people[ptr_two];

        if sum <= limit{
            ptr_one+=1;
            ptr_two-=1;
            boats+=1;
        }else if sum > limit{
            ptr_two-=1;
            boats+=1;
        }
    }

    if ptr_one == ptr_two{
        boats+=1;
    }

    return boats;        
}