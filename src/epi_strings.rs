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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mnemonics() {
        assert_eq!(
            letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }
}
