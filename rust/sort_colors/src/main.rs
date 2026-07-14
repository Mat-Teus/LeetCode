fn main() {
    let mut vec = vec![2,0,1];

    sort_colors(&mut vec);

    println!("{:?}", vec);
}

pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut count_0 = 0;
    let mut count_1 = 0;
    let mut count_2 = 0;

    for i in 0..nums.len(){
        if nums[i] == 0{
            count_0+=1;
        }else if nums[i] == 1{
            count_1+=1;
        }else if nums[i] == 2{
            count_2+=1;
        }
    }

    for i in 0..nums.len(){
        if count_0 > 0{
            nums[i] = 0;
            count_0 -= 1;
        }else if count_1 > 0{
            nums[i] = 1;
            count_1 -= 1;
        }else if count_2 > 0{
            nums[i] = 2;
            count_2 -= 1;
        }
    }   
}