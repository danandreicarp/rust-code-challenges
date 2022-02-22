use std::cmp::Ordering;

fn main() {
    let list: Vec<OrderedFloat> = vec![
        OrderedFloat(1.0),
        OrderedFloat(5.0),
        OrderedFloat(1.0),
        OrderedFloat(3.0),
        OrderedFloat(5.0),
    ];

    println!("Unique dataset: {:?}", unique(list));
}

fn unique<T: Ord>(list: Vec<T>) -> Vec<T> {
    let mut result: Vec<T> = Vec::with_capacity(list.len());

    for element in list.into_iter() {
        if !result.contains(&element) {
            result.push(element)
        }
    }
    result
}

#[derive(PartialEq, PartialOrd, Debug)]
struct OrderedFloat(f32);

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).expect("Trying to compare NaN")
    }
}

impl Eq for OrderedFloat {}

#[test]
fn empty_list() {
    let list: Vec<OrderedFloat> = vec![];
    let expected_result: Vec<OrderedFloat> = vec![];
    let actual_result = unique(list);
    assert_eq!(expected_result, actual_result);
}

#[test]
fn no_duplicates() {
    let list: Vec<OrderedFloat> = vec![OrderedFloat(1.0), OrderedFloat(4.0), OrderedFloat(5.0)];
    let expected_result = vec![OrderedFloat(1.0), OrderedFloat(4.0), OrderedFloat(5.0)];
    let actual_result = unique(list);
    assert_eq!(expected_result, actual_result);
}

#[test]
fn with_duplicates() {
    let list: Vec<OrderedFloat> = vec![OrderedFloat(1.0), OrderedFloat(1.0), OrderedFloat(3.0)];
    let expected_result = vec![OrderedFloat(1.0), OrderedFloat(3.0)];
    let actual_result = unique(list);
    assert_eq!(expected_result, actual_result);
}
