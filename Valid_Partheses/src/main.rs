use std::env;
struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack  = Vec::new();
        for c in s.chars() {
            let k = match c {
                '(' | '[' | '{' => {
                    stack.push(c);
                    true
                },
                ')' => {stack.pop() == Some('(')},
                ']' => {stack.pop() == Some('[')},
                '}' => {stack.pop() == Some('{')},
                unk => return false,
            };
            if !k {
                return false
            }

        }
        return stack.len() == 0 
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let s = "()[]{}".to_string();
    println!("{:?}",Solution::is_valid(s));

}
