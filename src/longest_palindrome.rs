// Tushar Roy explaining how to use Manacher's Algorithm to reduce time-O to O(n): https://www.youtube.com/watch?v=V-sEwsca1ak

use std::thread::current;

// Brute-force O(n^2)
// for each letter c at index i, which are less than max_palindrome_length away from the right end and more than max_palindrome_length+1 away from the left end
// check if s[i-max] == s[i+max]
// if yes, check i-max+1 == i+max-1, and so on
pub fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();

    if chars.is_empty() {
        return "".to_string();
    }

    if chars.len() == 1 {
        return chars[0].to_string();
    }

    // By defaul, max palindrome of a non-empty string is any single char
    let mut max_palindrome: Vec<char> = vec![chars[0]];

    for center in 1..chars.len() {
        
        let current_max = max_palindrome.len();

        // Break early if there is not enough letters to have a bigger palindrome centered on the next pivot
        if center + current_max / 2 >= chars.len() {
            break;
        }

        let mut no_change_of_max: bool = false;

        for j in 0..current_max/2 + 1 {
            if chars[center - j] != chars[center + j] {
                no_change_of_max = true;
                break;
            }
        }

        if !no_change_of_max {
            max_palindrome = chars[center - current_max / 2 - 1..center + current_max / 2 + 1].to_vec();
        }


    }

    max_palindrome.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(longest_palindrome("baba".to_string()), *"aba");
    }

    #[test]
    fn test_two() {
        assert_eq!(longest_palindrome("cbbd".to_string()), *"bb");
    }

    #[test]
    fn test_three() {
        assert_eq!(longest_palindrome("a".to_string()), *"a");
    }

    #[test]
    fn test_four() {
        assert_eq!(longest_palindrome("abcdef".to_string()), *"");
    }

    #[test]
    fn test_five() {
        assert_eq!(longest_palindrome("".to_string()), *"");
    }

    #[test]
    fn test_six() {
        assert!(longest_palindrome("babad".to_string()) == *"aba" || longest_palindrome("babad".to_string()) == *"bab");
    }
}