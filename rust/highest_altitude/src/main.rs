fn main() {
    let vec = vec![-4,-3,-2,-1,4,3,2];
    println!("{}", largest_altitude(vec));
}

pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut max_altitude = 0;
    let mut altitude = 0;

    for i in gain{
        altitude += i;

        if altitude > max_altitude{
            max_altitude = altitude;
        }
    }

    return max_altitude;
}