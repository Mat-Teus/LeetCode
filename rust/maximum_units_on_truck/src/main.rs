fn main() {
    let matrix = vec![
        vec![1,3],
        vec![2,2],
        vec![3,1],
    ];

    let n = 4;

    println!("{}", maximum_units(matrix, n));
}

pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
    let mut box_types = box_types;
    let mut truck_size = truck_size;
    let mut units = 0;
    
    box_types.sort_by_key(|unit| unit[1]);

    let mut j = 0;

    for i in (0..box_types.len()).rev(){
        while box_types[i][j] > 0{
            units+=box_types[i][j + 1];
            box_types[i][j] -= 1;
            truck_size -= 1;

            if truck_size == 0{
                break;
            }
        }

        if truck_size == 0{
            break;
        }
    }

    println!("{}", units);

    return units;        
}