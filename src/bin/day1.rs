use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;
use std::ops::AddAssign;
use std::str::FromStr;

fn main() {
    let content = fs::read_to_string("input/d1.txt").unwrap();
    let content = content
        .lines()
        .map(|s| s.split("  ").collect::<Vec<&str>>())
        .map(|x| (x[0], x[1]))
        .collect::<Vec<_>>();
    let mut list1 = vec![];
    let mut list2 = vec![];
    for (a, b) in content {
        println!("'{}' '{}'", a, b);
        list1.push(u32::from_str(a.trim()).unwrap());
        list2.push(u32::from_str(b.trim()).unwrap());
    }
    list1.sort();
    list2.sort();
    println!("{:#?}", list1);
    println!("{:#?}", list2);
    let total_distance: u32 = list1.iter().zip(list2.iter())
        .map(|(a, b)| max(a, b) - min(a, b))
        .sum();
    println!("Total distance: {}", total_distance);
    let mut appearance_map = HashMap::<u32, u32>::new();
    for no_to_test in list1 {
        for i in list2.clone() {
            if !appearance_map.contains_key(&no_to_test) {
                appearance_map.insert(no_to_test, 0);
            }
            let x = appearance_map.get_mut(&no_to_test)
                .unwrap();
            if i == no_to_test {
                x.add_assign(1);
            }
        }
    }
    let mut ss: u64 = 0;
    for (k, v) in appearance_map {
        println!("{} -> {} occurrences", k, v);
        ss += k as u64 * v as u64;
    }
    println!("ss {}", ss);
}