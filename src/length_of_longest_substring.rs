use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    
    let mut left_index = 0;
    let mut right_index = 0;
    let mut max_found = 0;
    
    let mut characters_seen: HashSet<char> = HashSet::new();
    let chars: Vec<char> = s.chars().collect();

    while right_index < chars.len() {
        while characters_seen.contains(&chars[right_index]) {
            characters_seen.remove(&chars[left_index]);
            left_index+=1;
        }
        
        characters_seen.insert(chars[right_index]);

        max_found = std::cmp::max(max_found, right_index - left_index + 1);

        right_index += 1;
    }
    
    max_found as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(length_of_longest_substring("".to_string()), 0);

        assert_eq!(length_of_longest_substring("aaa".to_string()), 1);
    }

    #[test]
    fn test_two() {
        assert_eq!(length_of_longest_substring("aaab".to_string()), 2);
    }

    #[test]
    fn test_three() {
        assert_eq!(length_of_longest_substring("baaab".to_string()), 2);
    }

}