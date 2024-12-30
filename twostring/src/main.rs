fn merge_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(nums1.len() + nums2.len());
    let mut i = 0;
    let mut j = 0;

    while i < nums1.len() && j < nums2.len() {
        if nums1[i] <= nums2[j] {
            result.push(nums1[i]);
            i += 1;
        } else {
            result.push(nums2[j]);
            j += 1;
        }
    }

    result.extend_from_slice(&nums1[i..]);
    result.extend_from_slice(&nums2[j..]);

    result
}

fn main() {
    let nums1 = [1, 3, 5, 7];
    let nums2 = [2, 4, 6, 8];

    let result = merge_sorted_arrays(&nums1, &nums2);
    println!("{:?}", result); // [1, 2, 3, 4, 5, 6, 7, 8]
}