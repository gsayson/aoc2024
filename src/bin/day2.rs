use std::fs;
use std::str::FromStr;

fn main() {
    let content = fs::read_to_string("input/d2.txt").unwrap();
    let reports = content.lines();
    let x = reports.map(|report| report.split_whitespace().map(|x| u32::from_str(x.trim()).unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
    let mut sc = 0u32;
    for vec in x {
        println!("{:?}", vec);
        let mut safe = true;
        for i in 1..vec.len() {
            let diff = vec[i].abs_diff(vec[i-1]);
            if diff < 1 || diff > 3 {
                safe = false;
                break
            }
        }
        let mut v = vec.clone();
        v.sort();
        let mut w = v.clone();
        w.reverse();
        if safe && (vec == v || vec == w) {
            sc += 1;
        }
        for j in 0..vec.len() {
            // rm 1 el
            let mut vec = vec.clone();
            vec.remove(j);
            println!("{:?}", vec);
            let mut safe = true;
            for i in 1..vec.len() {
                let diff = vec[i].abs_diff(vec[i-1]);
                if diff < 1 || diff > 3 {
                    safe = false;
                    break
                }
            }
            let mut v = vec.clone();
            v.sort();
            let mut w = v.clone();
            w.reverse();
            if safe && (vec == v || vec == w) {
                sc += 1;
                break
            }
        }
    }
    println!("{}", sc);
}