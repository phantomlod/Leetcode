struct Solution;

impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort();
        piles.reverse();
        let n = piles.len();
        let mut sum = 0 ;
        // if n >=3 && n<= 100000 && n%3 ==0 {
            for i in piles.iter() {
                if *i < 1 && *i >  1000 {
                    return 0;
                }
            }
            let numberPiles = (n / 3) as i32;
            let mut count = 1 ;
            for (k,v) in piles.iter().enumerate() {
                if count <= numberPiles && k%2 == 1 {
                    sum = sum + v;
                    count = count + 1;
                }
            }
            return sum;
            

        // } else {
        //     return 0;
        // }
        
    }
}
fn main() {
    let piles = vec![9,8,7,6,5,1,2,3,4];
    println!("{}", Solution::max_coins(piles));
}
