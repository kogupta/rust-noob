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

    // capture current context
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

                // reset context
                curr_string = String::new();
                k = 0;
            }
            ']' => {
                // eval expression
                // a2[c] -> a + 2 * c
                // a -> prev from stack, c -> current

                let n = numbers.pop().unwrap();
                let prev: &mut String = &mut string_stack.pop().unwrap();
                prev.push_str(curr_string.repeat(n).as_str());

                // update curr context
                curr_string = prev.to_string();
            }
            '0'..='9' => k = k * 10 + (c as u8 - b'0') as usize,
            _ => curr_string.push(c),
        }
    }

    curr_string
}

// https://leetcode.com/problems/longest-valid-parentheses
fn longest_valid_parentheses(s: String) -> i32 {
    use std::cmp::max;

    let mut expr_start: Vec<i32> = Vec::new();
    let mut len = 0;
    expr_start.push(-1);

    for (idx, c) in s.chars().enumerate() {
        if c == '(' {
            expr_start.push(idx as i32);
        } else {
            expr_start.pop();
            if expr_start.is_empty() {
                // no empty open parens - reset here
                expr_start.push(idx as i32);
            } else {
                let last = expr_start[expr_start.len() - 1];
                len = max(len, idx as i32 - last);
            }
        }
    }

    len
}

fn next_greater_element(items: Vec<i32>) -> Vec<i32> {
    // find next greater element
    // 2, 1, 3, 2, 4, 3
    // 3, 3, 4, 4, -1, -1  -> -1 is default for no greater number exists to the right

    // use a stack to track indices of input elements
    // while iterating input -
    //      if current number > stack[top] => current number > some previous number

    let mut result: Vec<i32> = vec![-1; items.len()];

    let mut indices: Vec<usize> = vec![];

    for (idx, n) in items.iter().enumerate() {
        while !indices.is_empty() && *n > items[*indices.last().unwrap()] {
            let last: usize = indices.pop().unwrap();
            result[last] = *n;
        }

        indices.push(idx);
    }

    result
}

fn next_smaller_element(items: Vec<i32>) -> Vec<i32> {
    let mut result = vec![-1; items.len()];
    let mut indices: Vec<usize> = vec![];
    for (idx, curr) in items.iter().enumerate() {
        while !indices.is_empty() && *curr < items[*indices.last().unwrap()] {
            let last: usize = indices.pop().unwrap();
            result[last] = *curr;
        }
        indices.push(idx);
    }

    result
}

// https://leetcode.com/problems/daily-temperatures
fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; temperatures.len()];
    let mut indices: Vec<usize> = vec![];

    for (idx, temp) in temperatures.iter().enumerate() {
        while !indices.is_empty() && *temp > temperatures[*indices.last().unwrap()] {
            let last_idx = indices.pop().unwrap();
            result[last_idx] = (idx - last_idx) as i32;
        }

        indices.push(idx);
    }

    result
}

// https://leetcode.com/problems/largest-rectangle-in-histogram/
fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    use std::cmp::max;

    // track (startIndex, value) in stack
    // stack has heights in increasing order
    // pop from stack when head >= curr item
    // say, stack has 2, 8 - now current height is 5
    //   => 2 will have width 3
    //   => 8 will have width 1
    //   => 5 will have width 2 - although starts at i, its starting index is mapped to that of 8

    let mut stack: Vec<(i32, i32)> = vec![];
    let mut max_area = 0;

    for (idx, height) in heights.iter().enumerate() {
        let idx = idx as i32;
        let mut pair = (idx, *height);

        loop {
            if stack.is_empty() {
                break;
            }

            if let Some((_, prev_height)) = stack.last() {
                if *height > *prev_height {
                    break;
                } else {
                    let (start_index, prev_height) = stack.pop().unwrap();
                    pair.0 = start_index;

                    max_area = max(max_area, prev_height * (idx - start_index));
                }
            }
        }

        stack.push(pair);
    }

    // check if any existing items have greater area
    let len = heights.len() as i32;
    for (start_index, height) in stack {
        max_area = max(max_area, height * (len - start_index));
    }

    max_area
}

#[cfg(test)]
mod tests {
    use crate::stacks::*;

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

    #[test]
    fn test_longest_valid_parentheses() {
        assert_eq!(longest_valid_parentheses("()(()".to_string()), 2);
        assert_eq!(longest_valid_parentheses("())))".to_string()), 2);
        assert_eq!(longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(longest_valid_parentheses(")()())((()()())".to_string()), 8);
    }

    #[test]
    fn test_next_greater_element() {
        assert_eq!(next_greater_element(vec![1, 2, 3]), vec![2, 3, -1]);
        assert_eq!(
            next_greater_element(vec![2, 1, 3, 2, 4, 3]),
            vec![3, 3, 4, 4, -1, -1]
        );
    }

    #[test]
    fn test_next_smaller_element() {
        assert_eq!(next_smaller_element(vec![1, 2, 3]), vec![-1, -1, -1]);
        assert_eq!(
            next_smaller_element(vec![2, 1, 3, 2, 4, 3]),
            vec![1, -1, 2, -1, 3, -1]
        );
    }

    #[test]
    fn test_daily_temperatures() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
        assert_eq!(daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
    }

    #[test]
    fn test_largest_rectangle_area() {
        assert_eq!(largest_rectangle_area(vec![2, 8, 5, 6, 3]), 15);
        assert_eq!(largest_rectangle_area(vec![2, 8, 5, 6, 2, 3]), 15);
        assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(largest_rectangle_area(vec![2, 4]), 4);
    }
}
