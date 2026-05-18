fn main() {
    
}

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut vec3 = Vec::new();
    let mut i:usize = 0;
    let mut j:usize = 0;

    while i < m as usize && j < n as usize{
        if nums1[i] >= nums2[j]{
            vec3.push(nums2[j]);
            j+=1;
        }else{
            vec3.push(nums1[i]);
            i+=1;
        }
    }

    while i < m as usize{
        vec3.push(nums1[i]);
        i+=1;
    } 

    while j < n as usize{
        vec3.push(nums2[j]);
        j+=1
    }

    *nums1 = vec3.clone();
}