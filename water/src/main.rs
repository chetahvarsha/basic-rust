pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        let width = (right - left) as i32;
        let h = if height[left] < height[right] {
            height[left]
        } else {
            height[right]
        };
        let area = width * h;
        max_area = std::cmp::max(max_area, area);
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}

fn main(){
    let x = [23,15,6,7,9,40,50];
    let x_vec: Vec<i32> = x.to_vec();
    let result = max_area(x_vec);
    println!("result = {}",result);
}