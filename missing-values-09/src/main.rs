fn sum_with_missing(numbers: Vec<Option<i32>>) -> i32 {
    // let mut subtotal = 0;
    // for n in numbers.iter() {
    //     if let Some(n) = n {
    //         subtotal += n;
    //     }
    // }
    // subtotal;

    // let mut sum = 0;
    // for ele in numbers {
    //     match ele {
    //         Some(v) => sum += v,
    //         None => continue,
    //     }
    // }
    // sum

    // numbers
    //     .iter()
    //     .filter(|ele| if let Some(_) = ele { true } else { false })
    //     .map(|ele| ele.unwrap())
    //     .sum()

    numbers.iter().map(|x| x.unwrap_or(0)).sum()
}

fn main() {
    let numbers = vec![Some(1), None, Some(2), None, Some(3), None, None, Some(4)];
    println!("sum with missing {:?}", sum_with_missing(numbers));
}

#[test]
fn empty() {
    let nn = vec![];
    assert_eq!(sum_with_missing(nn), 0);
}

#[test]
fn no_missing() {
    let nn = vec![Some(1), Some(5), Some(4)];
    assert_eq!(sum_with_missing(nn), 10);
}

#[test]
fn some_missing() {
    let nn = vec![None, Some(1), Some(5), Some(4), None, None];
    assert_eq!(sum_with_missing(nn), 10);
}

#[test]
fn all_missing() {
    let nn = vec![None, None, None];
    assert_eq!(sum_with_missing(nn), 0);
}
