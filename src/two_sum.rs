use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut elements: HashMap<i32, i32> = HashMap::new();
    
    for (idx, i) in nums.iter().enumerate() {            
        let current_idx = idx.try_into().unwrap();
        
        if elements.contains_key(&(target - i)) {
            let diff = target - i;
            return vec![current_idx, elements[&diff]];
        }

        elements.insert(*i, current_idx);
    }
    
    vec![0,0]
}

#[test]
fn test_one() {
    assert!(two_sum(vec![2, 7, 11, 15], 9) == vec![0, 1] || two_sum(vec![2, 7, 11, 15], 9) == vec![1, 0]);
}

#[test]
fn test_two() {
    assert!(two_sum(vec![3,2,4], 6) == vec![1,2] || two_sum(vec![3,2,4], 6) == vec![2,1]);
}

#[test]
fn test_three() {
    assert!(two_sum(vec![3,3], 6) == vec![1,0] || two_sum(vec![3,3], 6) == vec![0,1]);
}