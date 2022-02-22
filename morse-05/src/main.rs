#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}

/// Represents a single character
type Letter = Vec<Pulse>;

/// Represents a string of characters
type Message = Vec<Letter>;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        use Pulse::*;
        // let mut message = Vec::with_capacity(self.len());
        let mut message = Message::with_capacity(self.len());

        for mut ch in self.chars() {
            ch.make_ascii_lowercase();
            let letter = match ch {
                // 'a' => vec![Long, Short],
                'a' => Letter::from([Long, Short]),
                'b' => Letter::from([Long, Short, Short, Short]),
                'c' => Letter::from([Long, Short, Long, Short]),
                'd' => Letter::from([Long, Short, Short]),
                'e' => Letter::from([Short]),
                'f' => Letter::from([Short, Short, Long, Short]),
                'g' => Letter::from([Long, Long, Short]),
                'h' => Letter::from([Short, Short, Short, Short]),
                'i' => Letter::from([Short, Short]),
                'j' => Letter::from([Short, Long, Long, Long]),
                'k' => Letter::from([Long, Short, Long]),
                'l' => Letter::from([Short, Long, Short, Short]),
                'm' => Letter::from([Long, Long]),
                'n' => Letter::from([Long, Short]),
                'o' => Letter::from([Long, Long, Long]),
                'p' => Letter::from([Short, Long, Long, Short]),
                'q' => Letter::from([Long, Long, Short, Long]),
                'r' => Letter::from([Short, Long, Short]),
                's' => Letter::from([Short, Short, Short]),
                't' => Letter::from([Long]),
                'u' => Letter::from([Short, Short, Long]),
                'v' => Letter::from([Short, Short, Short, Long]),
                'w' => Letter::from([Short, Long, Long]),
                'x' => Letter::from([Long, Short, Short, Long]),
                'y' => Letter::from([Long, Short, Long, Long]),
                'z' => Letter::from([Long, Long, Short, Short]),
                '1' => Letter::from([Short, Long, Long, Long, Long]),
                '2' => Letter::from([Short, Short, Long, Long, Long]),
                '3' => Letter::from([Short, Short, Short, Long, Long]),
                '4' => Letter::from([Short, Short, Short, Short, Long]),
                '5' => Letter::from([Short, Short, Short, Short, Short]),
                '6' => Letter::from([Long, Short, Short, Short, Short]),
                '7' => Letter::from([Long, Long, Short, Short, Short]),
                '8' => Letter::from([Long, Long, Long, Short, Short]),
                '9' => Letter::from([Long, Long, Long, Long, Short]),
                '0' => Letter::from([Long, Long, Long, Long, Long]),
                _ => continue,
            };
            message.push(letter);
        }

        message
    }
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}

fn print_morse_code(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        }
        print!(" ");
    }
    println!();
}

fn main() {
    let greeting = "Hello, world".to_string().to_morse_code();

    print_morse_code(&greeting);
}

#[test]
fn hello_world() {
    use Pulse::*;

    let expected = vec![
        vec![Short, Short, Short, Short],
        vec![Short],
        vec![Short, Long, Short, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Long, Long],
        vec![Short, Long, Long],
        vec![Long, Long, Long],
        vec![Short, Long, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Short, Short],
    ];

    let actual = "Hello, world".to_string().to_morse_code();
    assert_eq!(actual, expected);
}

#[test]
fn whole_alphabet() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz1234567890".to_string();

    alphabet.to_morse_code();
    alphabet.to_uppercase().to_morse_code();

    assert_eq!("abcdefghijklmnopqrstuvwxyz1234567890", alphabet)
}
