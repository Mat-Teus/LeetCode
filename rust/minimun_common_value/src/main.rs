fn main() {
    let vec1 = vec![1,2,3,6];
    let vec2 = vec![2,3,4,5];

    println!("{}", get_common(vec1, vec2));
}

pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut j = 0;
    let mut i = 0;

    while i < nums1.len() && j < nums2.len(){
        if nums1[i] == nums2[j]{
            return nums1[i];
        }else if nums1[i] > nums2[j]{
            j+=1;
        }else{
            i+=1;   
        }
    }

    return -1;     
}