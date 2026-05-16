fn main() {
    let vec1 = vec![1,2,8,20, 30];
    let vec2 = vec![3,4,5,10, 15];

    println!("{}", find_median_sorted_arrays(vec1, vec2));
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut vec3 = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < nums1.len() && j < nums2.len(){
        if nums1[i] >= nums2[j]{
            vec3.push(nums2[j]);
            j+=1;
        }else{
            vec3.push(nums1[i]);
            i+=1;
        }
    }

    while i < nums1.len(){
        vec3.push(nums1[i]);
        i+=1;
    } 

    while j < nums2.len(){
        vec3.push(nums2[j]);
        j+=1
    }

    if vec3.len() %2 != 0{
        return *vec3.get(vec3.len()/2).unwrap() as f64;
    }else{
        println!("{:?}", vec3);
        return (*vec3.get(vec3.len()/2).unwrap() as f64 + *vec3.get(vec3.len()/2 - 1).unwrap() as f64)/2 as f64;
    }
}