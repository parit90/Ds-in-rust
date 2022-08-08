//https://leetcode.com/problems/palindrome-number/

struct Solution;

impl Solution{
    fn palindrome(x: i32) -> bool{
        if x<0 {
            return false
        }else{
            let mut y = x;
            let mut z = 0;
            while y >0{
                z = z*10;
                z += y%10;
                y = y/10;
            }
            z == x
        }
    }
}

fn main(){
    println!("{:?}",Solution::palindrome(122))
}