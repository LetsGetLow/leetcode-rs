/// less efficient solution than two pointer having O(n) time complexity and O(n) space complexity
/// the idea was to compare the first half of the string with the second half in reverse order.
/// but in order to determine the middle of the string, we need to filter non-alphanumeric bytes and collect a new string.
/// turns out that is in fact less efficient than comparing all bytes with eq or using two iterator pointers.
/// nain issue seems the extra allocation of the forward string.
///
/// This module provides functions to check if a given string is a valid palindrome.
///
/// # Arguments
///
/// * `s`: string to check if it is a palindrome
///
/// returns: bool
///
/// # Examples
/// ```
/// use leetcode_rs::_0125_valid_palindrome::is_palindrome_compare_half;
/// assert!(is_palindrome_compare_half("A man, a plan, a canal: Panama".to_string()));
pub fn is_palindrome_compare_half(s: String) -> bool {
    let forward: Vec<u8> = s
        .bytes()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let backward = forward.iter().rev();
    let half = forward.len() / 2;

    forward.iter().zip(backward).take(half).all(|(f, b)| f == b)
}

/// a bit less efficient solution than two pointer having O(n) time complexity and O(n) space complexity
/// the issue seems to be the extra allocation for the clone of the forward iterator.
///
/// # Arguments
///
/// * `s`: string to check if it is a palindrome
///
/// returns: bool
///
/// # Examples
///
/// ```
/// use leetcode_rs::_0125_valid_palindrome::is_palindrome_compare_all;
/// assert!(is_palindrome_compare_all("A man, a plan, a canal: Panama".to_string()));
/// ```
pub fn is_palindrome_compare_all(s: String) -> bool {
    let forward = s
        .bytes()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase());
    let backward = forward.clone().rev();

    forward.eq(backward)
}

/// most efficient solution having O(n) time complexity and O(1) space complexity
/// This could potentially even beat C++ solutions.
///
/// # Arguments
///
/// * `s`: string to check if it is a palindrome
///
/// returns: bool
///
/// # Examples
///
/// ```
/// use leetcode_rs::_0125_valid_palindrome::is_palindrome_two_pointer;
///
/// assert!(is_palindrome_two_pointer("A man, a plan, a canal: Panama".to_string()));
/// ```
pub fn is_palindrome_two_pointer(s: String) -> bool {
    let mut bytes = s.bytes().filter(|b| b.is_ascii_alphanumeric());
    while let (Some(left), Some(right)) = (bytes.next(), bytes.next_back()) {
        if left.eq_ignore_ascii_case(&right) {
            continue;
        }
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome_compare_half("racecar".to_string()));
        assert!(!is_palindrome_compare_half("race a car".to_string()));
        assert!(is_palindrome_compare_half(
            "A man, a plan, a canal: Panama".to_string()
        ));
        assert!(is_palindrome_compare_half(" ".to_string()));
    }

    #[test]
    fn test_is_palindrome_compare_all() {
        assert!(is_palindrome_compare_half("racecar".to_string()));
        assert!(!is_palindrome_compare_half("race a car".to_string()));
        assert!(is_palindrome_compare_half(
            "A man, a plan, a canal: Panama".to_string()
        ));
        assert!(is_palindrome_compare_half(" ".to_string()));
    }

    #[test]
    fn test_is_palindrome_two_pointer() {
        assert!(is_palindrome_compare_half("racecar".to_string()));
        assert!(!is_palindrome_compare_half("race a car".to_string()));
        assert!(is_palindrome_compare_half(
            "A man, a plan, a canal: Panama".to_string()
        ));
        assert!(is_palindrome_compare_half(" ".to_string()));
    }
}
