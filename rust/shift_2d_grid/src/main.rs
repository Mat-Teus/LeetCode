fn main() {
    let matrix = vec![
        vec![1,2,3],
        vec![4,5,6],
        vec![7,8,9],
    ];

    let k = 2;

    println!("{:?}", shift_grid(matrix, k));
}

pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut solution = vec![vec![0; grid[0].len()]; grid.len()];
    let mut i = 0;
    let mut j = 0;

    while i < grid.len(){
        while j < grid[i].len(){
            let index = i * grid[0].len() + j;
            let new_index = (index + k as usize) % (grid.len() * grid[0].len());
            let columns = grid[0].len();

            solution[new_index / columns][new_index % columns] = grid[i][j];
 
            j+=1;
        }
        i+=1;
        j = 0;
    }

    return solution;        
}