//https://leetcode.com/problems/two-sum/

struct Solution;
use std::collections::HashMap;

impl Solution{
    //brute force solution
    pub fn twoSum(nums: Vec<i32>, target: i32) -> Vec<i32>{
        for(i, num1) in nums.iter().enumerate(){
            for(j, num2) in nums.iter().skip(i+1).enumerate(){
                if num1+num2 == target{
                    return vec![i as i32, (i+j+1) as i32 ];
                }
            }
        }
        vec![]
    }

//logarthemic approach  
    pub fn method2(nums: Vec<i32>, target: i32) -> Vec<i32>{
        let mut _hashmap: HashMap<i32, usize> = HashMap::new();
        for(i, num1) in nums.iter().enumerate(){
            let temp = target - num1;
            if let Some(&temp_index) = _hashmap.get(&temp){
                return vec![temp_index as i32, i as i32];
            }
            _hashmap.insert(*num1,i);
        }
        vec![]
    }
}

fn main(){
    println!("{:?}",Solution::method2(vec![2,5,11,15], 7))
}