struct Solution {}


impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut result = Vec::new();

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum < 0 {
                    j += 1;
                } else if sum > 0 {
                    k -= 1;
                } else {

                    result.push(vec![nums[i], nums[j], nums[k]]);
                    while j < k && nums[j] == nums[j + 1] {
                        j += 1;
                    }
                    while j < k && nums[k] == nums[k - 1] {
                        k -= 1;
                    }
                    j += 1;
                    k -= 1;
                }
            }
        }

        result
    }
}

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];    //, 3, 5, 9, 3];
    let result = Solution::three_sum(nums);
    println!("{:?}", result);
}