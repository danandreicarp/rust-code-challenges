use std::fmt::{Debug, Formatter};
use std::str::FromStr;

const RADIX: u32 = 10;

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[allow(unused)]
#[derive(Debug)]
enum InvalidIsbn {
    TooLong,
    TooShort,
    FailedChecksum,
    InvalidCharacter(usize, char),
}

#[derive(Debug)]
struct IsbnError {
    message: String,
}

impl IsbnError {
    fn new(message: String) -> Self {
        IsbnError { message }
    }
}

// impl Debug for IsbnError {
//     fn fmt(&self, formatter: &mut Formatter) -> core::fmt::Result {
//         write!(formatter, "ISBN Error: {}", self.message)
//     }
// }

/*
impl FromStr for Isbn {
    type Err = InvalidIsbn;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::with_capacity(13);

        for (i, c) in s.char_indices() {
            match c {
                '-' => continue,
                '0'..='9' => digits.push(c.to_digit(10).unwrap() as u8),
                _ => {
                    return Err(InvalidIsbn::InvalidCharacter(i, c));
                }
            }
        }

        let n_digits = digits.len();

        if n_digits > 13 {
            return Err(InvalidIsbn::TooLong);
        } else if n_digits < 13 {
            return Err(InvalidIsbn::TooShort);
        }

        if digits[12] != calculate_check_digit(&digits) {
            return Err(InvalidIsbn::FailedChecksum);
        }

        Ok(Isbn {
            raw: s.to_string(),
            digits: digits,
        })
    }
}
*/

impl FromStr for Isbn {
    type Err = IsbnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace("-", "");

        if s.len() < 13 {
            Err(IsbnError::new(format!("input is too short: {}", s.len())))
        } else if s.len() > 13 {
            Err(IsbnError::new(format!("input is too long: {}", s.len())))
        } else {
            let iterator = s
                .chars()
                .take(s.len() - 1)
                .map(|n| n.to_digit(RADIX).expect("not numeric input") as u8);

            let mut digits: Vec<u8> = Vec::from_iter(iterator);
            let check_digit: u8 = s
                .chars()
                .last()
                .unwrap()
                .to_digit(RADIX)
                .expect("not numeric input") as u8;

            let check_result = calculate_check_digit(&digits);

            if check_digit == check_result {
                let mut last_element: Vec<u8> = vec![check_digit];
                digits.append(&mut last_element);
                Ok(Isbn {
                    raw: s,
                    digits: digits,
                })
            } else {
                Err(IsbnError::new(format!("input checksum failed for {}", s)))
            }
        }
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
// fn calculate_check_digit(digits: &[u8]) -> u8 {
//     const WEIGHTS: [u8; 12] = [1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];

//     let sum: u32 = digits
//         .iter()
//         .zip(WEIGHTS.iter())
//         .map(|(&x, &y)| x * y)
//         .map(|subtotal| subtotal as u32)
//         .sum();

//     let check_digit = 10 - (sum % 10);
//     match check_digit {
//         10 => 0_u8,
//         n => n as u8,
//     }
// }

fn calculate_check_digit(digits: &[u8]) -> u8 {
    let mut sum = 0;
    for i in 0..digits.len() {
        if i % 2 == 0 {
            sum += digits[i];
        } else {
            sum += digits[i] * 3;
        }
    }

    let result = 10 - (sum % 10);
    match result {
        10 => 0,
        n => n,
    }
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();
    println!("Rust in Action's ISBN-13 ({}) is valid!", rust_in_action);

    let other: Isbn = "978-1-86-197876-9".parse().unwrap();
    println!("Other ISBN-13 ({}) is valid!", other);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}
