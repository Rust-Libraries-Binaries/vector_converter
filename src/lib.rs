/// Converts a vector of integers to a vector of strings.
pub fn int_vec_to_string_vec(int_vec: Vec<i32>) -> Vec<String> {
    int_vec.into_iter().map(|x| x.to_string()).collect()
}

/// Converts a vector of strings to a vector of integers.
/// Returns an error if any string cannot be parsed as an integer.
pub fn string_vec_to_int_vec(string_vec: Vec<String>) -> Result<Vec<i32>, std::num::ParseIntError> {
    string_vec.into_iter().map(|s| s.parse()).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_vec_to_string_vec() {
        let int_vec = vec![1, 2, 3];
        let string_vec = int_vec_to_string_vec(int_vec);
        assert_eq!(string_vec, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_string_vec_to_int_vec() {
        let string_vec = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        let int_vec = string_vec_to_int_vec(string_vec).unwrap();
        assert_eq!(int_vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_string_vec_to_int_vec_invalid() {
        let string_vec = vec!["1".to_string(), "invalid".to_string()];
        let result = string_vec_to_int_vec(string_vec);
        assert!(result.is_err());
    }
}
