fn main() {
    let vec1 = vec![1,2,2,1];
    let vec2 = vec![2,2];

    println!("{:?}", intersect(vec1, vec2));
}

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut solution = Vec::new();
    let mut nums2 = nums2;
    let mut ptr_two = 0;

    for ptr_one in 0..nums1.len(){
        println!("{}", nums1[ptr_one]);

        for ptr_two in 0..nums2.len(){
            if nums1[ptr_one] == nums2[ptr_two]{
                nums2[ptr_two] = -1;
                solution.push(nums1[ptr_one]);
                break;
            }
        }
    }

    return solution;
}