fn main() {
    let matrix = vec![
        vec![5,1],
        vec![-5,-5],
        vec![-3,-3],
        vec![-3,-3],
    ];


    println!("{}", count_negatives(matrix));
}

pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    let mut i = 0;
    let mut j:i32 = (grid[0].len() - 1) as i32;

    while i < grid.len() && j>=0{
        if grid[i][j as usize] < 0{
            count += 1;
            j-=1;
                if j == -1{
                i+=1;
                j = (grid[0].len() - 1) as i32;
            }
        }else{
            i+=1;
            j = (grid[0].len() - 1) as i32;
        }
    }

    return count;
}