#[cfg(test)]
mod tests {
    use week7_rust_ds581::is_valid;
    
    
    #[test]
    fn test_bracket_validator_valid() {
        assert_eq!(is_valid("()"), true);
        assert_eq!(is_valid("(())"), true);
        assert_eq!(is_valid("()"), true);
        assert_eq!(is_valid("()()"), true);
        assert_eq!(is_valid("(()())"), true);
    }

    #[test]
    fn test_bracket_validator_invalid() {
        assert_eq!(is_valid("("), false);
        assert_eq!(is_valid(")"), false);
        assert_eq!(is_valid("(()"), false);
        assert_eq!(is_valid("())"), false);
        assert_eq!(is_valid("())))"), false);
        assert_eq!(is_valid("()((((()"), false);
    }
}

