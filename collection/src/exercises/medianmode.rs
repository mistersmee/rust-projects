use std::collections::HashMap;

pub fn median() {
    let v = [5, 3, 4, 1, 2];

    let mut median = Vec::new();

    for items in v {
        median.push(items);
    }

    median.sort();

    println!("The given list of integers is: {median:?}");

    if median.len() % 2 == 0 {
        println!("The median of the given list of integers is: {}", median[(median.len() / 2) - 1]);
    } else {
        println!("The median of the given list of integers is: {}", median[((median.len() + 1) / 2) - 1]);
    }
}

pub fn mode() {
    let v = [1, 1, 2, 2, 2, 3, 4, 5, 6];

    println!("The given list of integers is: {v:?}");

    let mut map = HashMap::new();

    for item in v {
        let count = map.entry(item).or_insert(0);
        *count += 1;
    }

    match map.values().max() {
        None => None,
        Some(i) => Some(println!("The mode of the given list of integers is: {i}")),
    };
}
