use std::ffi::CString;
use std::path::Path;

fn main() {
    let a: &str = "?";
    let b: String = "?".to_string();

    info(&a);
    info(&b);

    // Advanced 1
    let c = CString::new("?").unwrap();
    // info(&c.to_str().ok().unwrap());
    info(&c);

    // Advanced 2
    let d = Path::new("/tmp/linkedin-learning");
    // info(&d.display());
    info(&d);
}

// fn info<T: std::fmt::Display>(text: &T) {
//     println!("{}", text);
// }

// fn info<T: ToString>(text: &T) {
// println!("{}", text.to_string());
// }

// fn info<T: AsRef<str>>(text: &T) {
// println!("{}", text.as_ref());
// }

fn info<T: std::fmt::Debug>(text: &T) {
    println!("{:?}", text);
}

#[test]
fn str() {
    let input = "Rust";
    info(&input);
}

#[test]
fn string() {
    let input = String::from("Rust");
    info(&input);
}

#[test]
fn chars() {
    let input = 'r';
    info(&input);
}

#[test]
fn cstring() {
    let input = CString::new("Rust").unwrap();
    info(&input.to_str().ok().unwrap());
}

#[test]
fn path() {
    let input = Path::new("/tmp/rust");
    info(&input.display());
}
