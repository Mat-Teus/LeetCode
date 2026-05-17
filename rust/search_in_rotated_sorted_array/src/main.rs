fn main() {
    let vec = vec![3,4,5,6,1,2];
    let target = 2;

    println!("{}", search(vec, target));
}

pub fn search(nums: Vec<i32>, target: i32) -> i32{
    let mut begin:usize = 0;
    let mut end = nums.len() - 1;
    let mut middle = nums.len()/2;

    while begin < end{
        if nums[begin] <= nums[end]{
            break;
        }
        middle = begin + (end - begin)/2;

        if nums[middle] > nums[end]{
            begin = middle + 1;
        }else{
            end = middle;
        }

    }

    let mut pivot = begin;

    if nums[pivot] == target{
        return pivot as i32;
    }else if pivot != 0 && target >= nums[0]{
        begin = 0;
        if pivot == 0{
            pivot = 1;
        }
        end = pivot - 1;
        while begin <= end{
            middle = begin + (end - begin)/2;
            if nums[middle]==target{
                return middle as i32;
            }else if nums[middle] < target{
                begin = middle + 1;
            }else if nums[middle] > target{
                if middle == 0{
                    break;
                }
                end = middle - 1;
            }
        }
    }else{
        begin = pivot;
        end = nums.len() - 1;
        while begin <= end{
            middle = begin + (end - begin)/2;
            if nums[middle]==target{
                return middle as i32;
            }else if nums[middle] < target{
                begin = middle +1;
            }else if nums[middle] > target{
                if middle == 0{
                    break;
                }
                end = middle - 1;
            }
        }
    }

    return -1;
}