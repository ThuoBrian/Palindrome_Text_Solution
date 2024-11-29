fn main() {
    println!("Enter a word:");

    let mut word = String::new();

    std::io::stdin()
        .read_line(&mut word)
        .expect("Fail to read the entered word");

    is_palindrome_text(&word.trim());
}

fn is_palindrome_text(text: &str) -> bool {
    let x_transform = |x: &str| -> String { x.to_lowercase().chars().rev().collect() };
    let value_text = x_transform(text) == text.to_lowercase();
    value_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_even() {
        // Test for an even-length palindrome
        assert_eq!(is_palindrome_text("abba"), true);
    }

    #[test]
    fn test_palindrome_odd() {
        // Test for an odd-length palindrome
        assert_eq!(is_palindrome_text("racecar"), true);
    }

    #[test]
    fn test_not_palindrome() {
        // Test for a non-palindrome string
        assert_ne!(is_palindrome_text("hello"), true);
    }

    #[test]
    fn test_case_insensitive_palindrome() {
        // Test case insensitivity by passing mixed case input
        assert_eq!(is_palindrome_text("Madam"), true);
    }

    #[test]
    fn test_empty_string() {
        // Test with an empty string (empty string is considered a palindrome)
        assert_eq!(is_palindrome_text(""), true);
    }
}
