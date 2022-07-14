struct Solution ;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string()==x.to_string().chars().rev().collect::<String>()
    }
}

fn main() { 
    
    let a = Solution::is_palindrome(123456);
    
    println!("{:?}",a);
}