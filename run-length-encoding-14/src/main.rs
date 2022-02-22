mod run_length_encoding {
    // pub fn encode(text: &str) -> String {
    //     let mut count = 0;
    //     let mut prev: Option<char> = None;
    //     let mut encoded = String::with_capacity(text.len() / 2);
    //     let mut chars = text.chars();

    //     while let Some(c) = chars.next() {
    //         if prev.is_none() {
    //             prev = Some(c);
    //         }

    //         if prev.unwrap() != c || count == 9 {
    //             encoded.push_str(&format!("{}{}", count, prev.unwrap()));
    //             count = 0
    //         }
    //         prev = Some(c);
    //         count += 1;
    //     }

    //     // protect against empty string
    //     if let Some(prev) = prev {
    //         encoded.push_str(&format!("{}{}", count, prev));
    //     }
    //     encoded
    // }

    // pub fn decode(text: &str) -> String {
    //     let mut decoded = String::with_capacity(text.len() * 2);
    //     let mut chars = text.chars();

    //     while let (Some(n), Some(c)) = (chars.next(), chars.next()) {
    //         let n = n.to_digit(10).unwrap();
    //         for _ in 0..n {
    //             decoded.push(c);
    //         }
    //     }

    //     decoded
    // }

    pub fn encode(text: &str) -> String {
        let mut result: Vec<String> = Vec::new();
        let mut chars = text.chars().peekable();

        let mut count: u8 = 1;
        while let Some(ch) = chars.next() {
            match chars.peek() {
                Some(next_ch) => {
                    if *next_ch == ch {
                        if count == 9 {
                            save_encoding(&count, ch, &mut result);
                            count = 1;
                        } else {
                            count += 1;
                        }
                        continue;
                    } else {
                        save_encoding(&count, ch, &mut result);
                        count = 1;
                    }
                }
                None => {
                    save_encoding(&count, ch, &mut result);
                    break;
                }
            }
        }

        let mut output = String::new();
        for s in result {
            output.push_str(&s);
        }
        output
    }

    pub fn decode(text: &str) -> String {
        let mut result = String::new();

        let mut i = 0;
        while i < text.len() - 1 {
            let encoding = text.get(i..=i + 1).unwrap();
            let mut iter = encoding.chars();
            let count = iter.next().unwrap().to_digit(10).unwrap();
            let ch = iter.next().unwrap();

            for _ in 0..count {
                result.push(ch);
            }
            i = i + 2;
        }

        result
    }

    fn save_encoding(count: &u8, ch: char, result: &mut Vec<String>) {
        let mut encoding = count.to_string();
        encoding.push(ch);
        result.push(encoding);
    }
}

fn main() {
    //
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}

#[test]
fn long_run_round_trip() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(
        decode(&encode(input)),
        "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA"
    );
}
