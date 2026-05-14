fn main() {
    let vec = vec![2,2,2,3,3,3];
    let target = 2;

    println!("{:?}", search_range(vec, target));
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = vec![-1, -1];
    let mut begin = 0;
    let mut end = nums.len();
    let mut middle = nums.len()/2;

    while begin < end{
        if nums[middle] == target{
            result.clear();
            while middle > 0 && nums[middle - 1] == target{
                middle -= 1;
            }
            result.push((middle) as i32);
            break;
        }else if nums[middle] < target{
            begin = middle + 1;
        }else if nums[middle] > target{
            end = middle;
        }
        middle = begin + (end - begin)/2;
    }

    let mut begin = 0;
    let mut end = nums.len();
    let mut middle = nums.len()/2;

    while begin < end{
        if nums[middle] == target{
            while middle < nums.len() && nums[middle] == target{
                middle += 1;
            }
            result.push((middle - 1) as i32);
            break;
        }else if nums[middle] < target{
            begin = middle + 1;
        }else if nums[middle] > target{
            end = middle;
        }
        middle = begin + (end - begin)/2;
    }



    return result;
}