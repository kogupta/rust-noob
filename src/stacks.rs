
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

// https://leetcode.com/problems/decode-string/
fn decode_string(s: String) -> String {
    // example: x3[a[2[c]]y => xaccaccaccy
    // stack - [ -> capture context for expr evaluation
    //         ] -> eval expression, push expr back on stack/context

    // these capture current context
    let mut k: usize = 0;
    let mut curr_string = String::new();

    let mut string_stack: Vec<String> = Vec::new();
    let mut numbers: Vec<usize> = Vec::new();

    for c in s.chars() {
        match c {
            '[' => {
                // capture context
                numbers.push(k);
                string_stack.push(curr_string.clone());
                println!("[start] stacks: {:?}, {:?}", string_stack, numbers);

                // reset context
                curr_string = String::new();
                k = 0;
            },
            ']' => {
                // eval expression
                // a2[c] -> a + 2 * c
                // a -> prev from stack, c -> current
                println!("[>eval] stacks: {:?}, {:?}", string_stack, numbers);

                let n = numbers.pop().unwrap();
                let prev: &mut String = &mut string_stack.pop().unwrap();
                prev.push_str(curr_string.repeat(n).as_str());

                // update curr context
                curr_string = prev.to_string();
                println!("[<eval] stacks: {:?}, {:?}, {:?}",
                         string_stack, numbers, curr_string);
            },
            '0'..='9' => k = k * 10 + (c as u8 - b'0') as usize,
            _ => curr_string.push(c),
        }
    }

    curr_string
}

#[cfg(test)]
mod tests {
    use crate::stacks::{decode_string, is_valid};

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

    #[test]
    fn test_decode_string() {
        assert_eq!(decode_string("2[ab]".to_string()), "abab");
        assert_eq!(decode_string("2[ab]3[c]".to_string()), "ababccc");
        assert_eq!(decode_string("x2[ab]3[c]".to_string()), "xababccc");
        assert_eq!(decode_string("2[ab]3[c]y".to_string()), "ababcccy");
        assert_eq!(decode_string("x2[ab]3[c]y".to_string()), "xababcccy");
        assert_eq!(decode_string("3[a2[c]]".to_string()), "accaccacc");
        assert_eq!(decode_string("x3[a2[c]]y".to_string()), "xaccaccaccy");
    }
}
