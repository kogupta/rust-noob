fn count_and_say(n: i32) -> String {
    fn next(input: &[usize], output: &mut Vec<usize>) {
        println!("input: {:?}, buffer: {:?}", input, output);

        // NOTE: DO NOT use rusty for-loop
        // https://stackoverflow.com/a/70137058
        let mut idx = 0;
        let len = input.len();
        while idx < len {
            let mut count: usize = 1;
            let digit = input[idx];
            while idx < len - 1 && input[idx + 1] == digit {
                count += 1;
                idx += 1;
            }
            output.push(count);
            output.push(digit);

            idx += 1;
        }
    }

    let mut buf: Vec<usize> = vec![1];
    let mut acc: Vec<usize> = vec![];
    for _ in 2..=n {
        next(&buf, &mut acc);
        buf = acc.clone();
        acc.clear();
    }

    buf.iter()
        .map(|digit| (*digit as u8 + b'0') as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_say() {
        assert_eq!(count_and_say(1), "1");
        assert_eq!(count_and_say(2), "11");
        assert_eq!(count_and_say(3), "21");
        assert_eq!(count_and_say(4), "1211");
        assert_eq!(count_and_say(5), "111221");
        assert_eq!(count_and_say(6), "312211");
        assert_eq!(count_and_say(7), "13112221");
        assert_eq!(count_and_say(8), "1113213211");
        assert_eq!(count_and_say(9), "31131211131221");
    }
}
