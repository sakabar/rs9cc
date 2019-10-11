pub fn strtol(s: &String, radix: u32) -> Result<(i64, String), &'static str> {
    if radix > 36 {
        return Err("too big radix (> 36)");
    }

    let mut last_read = None;
    let mut chars = s.chars();
    let ans_s: String = chars
        .by_ref()
        .inspect(|ch| last_read = Some(ch.to_string()))
        .take_while(|ch| ch.is_digit(radix))
        .fold(String::new(), |mut s, c| {
            s.push(c);
            return s;
        });

    if ans_s == "" {
        return Err("No number is parsed.");
    }

    let mut rest_s = String::new();

    let next_ch: Option<char> = chars.next();
    // charsがまだ残っている場合は、take_whileで無駄にした1文字とチェックに使った1文字を戻す
    if next_ch.is_some() {
        rest_s.push_str(&last_read.unwrap_or(String::new()));
        rest_s.push(next_ch.unwrap());
    }

    rest_s.push_str(chars.as_str());

    let ans_num = i64::from_str_radix(ans_s.as_str(), radix).ok().unwrap();
    Ok((ans_num, rest_s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strtol_1() {
        let (actual_num, actual_rest_s) = strtol(&"ff 1".to_string(), 16).ok().unwrap();
        let expected_num = 255;
        let expected_rest_s = " 1";
        assert_eq!(actual_num, expected_num);
        assert_eq!(actual_rest_s.as_str(), expected_rest_s);
    }

    #[test]
    fn test_strtol_2() {
        let result = strtol(&"aaa".to_string(), 10);
        assert_eq!(result, Err("No number is parsed."));
    }

    #[test]
    fn test_strtol_3() {
        let (actual_num, actual_rest_s) = strtol(&"42".to_string(), 10).ok().unwrap();
        let expected_num = 42;
        let expected_rest_s = "";
        assert_eq!(actual_num, expected_num);
        assert_eq!(actual_rest_s.as_str(), expected_rest_s);
    }
}
