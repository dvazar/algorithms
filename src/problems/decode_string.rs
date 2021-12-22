/// https://leetcode.com/problems/decode-string/
/// Constraints:
//    1 <= s.length <= 30
//    s consists of lowercase English letters, digits, and square brackets '[]'.
//    s is guaranteed to be a valid input.
//    All the integers in s are in the range [1, 300].
pub fn decode_string(s: String) -> String {
    let mut decoded_string = String::new();

    let mut k = String::new();
    let mut encoded_string = String::new();
    let mut n_brackets = 0u8;
    let mut max_n_brackets = 0u8;

    for char in s.chars().into_iter() {
        if char.is_digit(10) {
            match n_brackets {
                0 => k.push(char),
                _ => encoded_string.push(char)
            }
        }
        else if char == '[' {
            if n_brackets > 0 {encoded_string.push(char)}
            n_brackets += 1;
            max_n_brackets += 1;
        }
        else if char == ']' {
            n_brackets -= 1;
            match n_brackets {
                0 => {
                    match max_n_brackets {
                        1 => decoded_string.push_str(
                            encoded_string.repeat(
                                k.parse::<usize>().unwrap()
                            ).as_str()
                        ),
                        _ => decoded_string.push_str(
                            decode_string(
                                encoded_string.clone()
                            ).repeat(
                                k.parse::<usize>().unwrap()
                            ).as_str()
                        )
                    }
                    k.clear();
                    encoded_string.clear();
                    max_n_brackets = 0;
                },
                _ => encoded_string.push(char)
            }
        }
        else {
            match n_brackets {
                0 => decoded_string.push(char),
                _ => encoded_string.push(char)
            }
        }
    }

    decoded_string
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_string_1() {
        assert_eq!(decode_string("3[a]2[bc]".to_string()), "aaabcbc");
    }

    #[test]
    fn test_decode_string_2() {
        assert_eq!(decode_string("3[a2[c]]".to_string()), "accaccacc");
    }

    #[test]
    fn test_decode_string_3() {
        assert_eq!(decode_string("2[abc]3[cd]ef".to_string()), "abcabccdcdcdef");
    }

    #[test]
    fn test_decode_string_4() {
        assert_eq!(decode_string("abc3[cd]xyz".to_string()), "abccdcdcdxyz");
    }

}
