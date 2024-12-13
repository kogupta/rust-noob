fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let digit_char_map: [&[char]; 10] = [
        &*vec![],
        &*vec![],
        &*vec!['a', 'b', 'c'],
        &*vec!['d', 'e', 'f'],
        &*vec!['g', 'h', 'i'],
        &*vec!['j', 'k', 'l'],
        &*vec!['m', 'n', 'o'],
        &*vec!['p', 'q', 'r', 's'],
        &*vec!['t', 'u', 'v'],
        &*vec!['w', 'x', 'y', 'z'],
    ];
    fn dfs(
        digits: &Vec<usize>,
        digit_char_map: [&[char]; 10],
        index: usize,
        buffer: &mut String,
        acc: &mut Vec<String>,
    ) {
        if index == digits.len() {
            acc.push(buffer.clone());
            return;
        }

        for c in digit_char_map[digits[index]] {
            buffer.push(*c);
            dfs(digits, digit_char_map, index + 1, buffer, acc);
            buffer.pop();
        }
    }

    let digits: Vec<usize> = digits
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    let mut result: Vec<String> = {
        let mut len = 1;
        for c in &digits {
            let options = &digit_char_map[*c].len();
            len *= options;
        }

        Vec::with_capacity(len)
    };

    dfs(
        &digits,
        digit_char_map,
        0,
        &mut String::with_capacity(digits.len()),
        &mut result,
    );

    result
}

fn letter_combinations_iterative(digits: String) -> Vec<String> {
    use std::collections::VecDeque;

    if digits.is_empty() {
        return vec![];
    }

    let digit_char_map: [&[char]; 10] = [
        &*vec![],
        &*vec![],
        &*vec!['a', 'b', 'c'],
        &*vec!['d', 'e', 'f'],
        &*vec!['g', 'h', 'i'],
        &*vec!['j', 'k', 'l'],
        &*vec!['m', 'n', 'o'],
        &*vec!['p', 'q', 'r', 's'],
        &*vec!['t', 'u', 'v'],
        &*vec!['w', 'x', 'y', 'z'],
    ];

    let digits: Vec<usize> = digits
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    let mut result: Vec<String> = {
        let mut len = 1;
        for c in &digits {
            let options = &digit_char_map[*c].len();
            len *= options;
        }

        Vec::with_capacity(len)
    };

    // use bfs -> "level" indicates the index being processed
    let mut q: VecDeque<String> = VecDeque::new();
    let mut index = 0;
    for c in digit_char_map[digits[0]] {
        let mut buffer = String::with_capacity(digits.len());
        buffer.push(*c);
        q.push_back(buffer);
    }

    while !q.is_empty() {
        let size = q.len();
        index += 1;
        for _ in 0..size {
            if let Some(buffer) = q.pop_front() {
                if index == digits.len() {
                    result.push(buffer);
                } else {
                    for c in digit_char_map[digits[index]] {
                        let mut buf = buffer.clone();
                        buf.push(*c);
                        q.push_back(buf);
                    }
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mnemonics() {
        assert_eq!(
            letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert_eq!(
            letter_combinations_iterative("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );

        let numbers = [2, 48, 798, 4569];
        for number in numbers {
            assert_eq!(
                letter_combinations(number.to_string()),
                letter_combinations_iterative(number.to_string())
            );
        }
    }
}
