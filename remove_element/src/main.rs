fn remove_element(nums: &mut Vec<i32>, val: i32) -> usize {
    let mut k = 0;

    // Use a mutable iterator to modify the vector in place
    let mut i = 0;
    while i < nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
        i += 1;
    }

    // Return the number of elements not equal to val
    k
}

fn main() {
    let mut nums = vec![3, 2, 2, 3, 4, 5, 6];
    let val = 3;

    let k = remove_element(&mut nums, val);

    println!("Number of elements not equal to {}: {}", val, k);
    println!("Modified array: {:?}", &nums[..k]);
}
