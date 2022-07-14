use std::env;
struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        
        let mut res = 0 ;
        if nums[0] > target {
            return 0
        } else if nums[nums.len()-1] < target {
            return nums.len()  as i32 
        } else {
            for i in 1..nums.len() {
                
                if nums[i-1] < target && nums[i] >= target {
                    res = i;
                }
            }
        }

        res as i32
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let a = vec![1,3,5,6];
    println!("{}",Solution::search_insert(a, 2));
}
