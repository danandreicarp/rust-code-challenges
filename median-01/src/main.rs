fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);
    println!("median[1,2,5]) = {:?}", answer);
}

fn median(mut list: Vec<f32>) -> Option<f32> {
    if list.is_empty() {
        return None;
    }

    let n_elements = list.len();
    let middle = n_elements / 2;

    list.sort_by(|a, b| {
        a.partial_cmp(b).unwrap()

        // if a < b {
        //     return Ordering::Less;
        // } else if a > b {
        //     return Ordering::Greater;
        // } else {
        //     return Ordering::Equal;
        // }
    });

    println!("sorted vector: {:?}", list);

    let med = if n_elements % 2 == 1 {
        list[middle]
    } else {
        (list[middle - 1] + list[middle]) / 2.0
    };

    Some(med)
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![3.0, 1.5, 8.8, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
