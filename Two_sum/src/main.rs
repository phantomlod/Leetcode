
struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{
        use std::collections::HashMap;
        let mut hash : HashMap<i32,i32> = HashMap::new();
        for (i, value) in nums.iter().enumerate(){
            
            match hash.get(&(target - *value)) {
                Some(&result_target) => return vec![result_target, i as i32],
                None => hash.insert(*value , i as i32) ,
            };

            println!("{:?}",*value);
        }
        
        vec![]
    }
   
}

fn main() {
    let test = Solution::two_sum(vec![3,2,4], 6);
    println!("{:?}",test);
}