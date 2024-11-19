// https://leetcode.com/problems/valid-parentheses/
fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        if c == '(' {
            stack.push(')');
        } else if c == '{' {
            stack.push('}');
        } else if c == '[' {
            stack.push(']');
        } else if stack.is_empty() || stack.pop().unwrap() != c {
            return false;
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use crate::stacks::is_valid;

    #[test]
    fn test_is_valid() {
        assert!(is_valid("()".to_string()));
        assert!(is_valid("()[]{}".to_string()));
        assert!(!is_valid("(]".to_string()));
        assert!(is_valid("([])".to_string()));
        assert!(!is_valid("(){({}})".to_string()));
        assert!(is_valid("(){({})}".to_string()));
        assert!(!is_valid("(){}}{".to_string()));
    }
}