use std::io;

fn main() {
    let mut stdin_iter = io::stdin().lines();
    let _capacity: usize = stdin_iter
        .next()
        .and_then(|s| s.ok()?.parse::<usize>().ok())
        .unwrap();

    let line = stdin_iter.next().unwrap().unwrap();

    let mut iter = line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .enumerate();

    let first = iter.next().unwrap();
    let (mut min, mut max) = (first, first);

    let (mut i1, mut i2, mut j1, mut j2) = (0usize, 0usize, 0usize, 0usize);
    let (mut best_min, mut best_max) = (i32::MAX, i32::MIN);

    for (j, x) in iter {
        let d1 = min.1 - x;
        let d2 = max.1 - x;

        if d1 < best_min || (d1 == best_min && min.0 < i1) {
            best_min = d1;
            i1 = min.0;
            j1 = j;
        }

        if d2 > best_max || (d2 == best_max && max.0 < i2) {
            best_max = d2;
            i2 = max.0;
            j2 = j;
        }

        if x < min.1 {
            min = (j, x);
        }

        if x > max.1 {
            max = (j, x);
        }
    }

    println!("{} {}", i1 + 1, j1 + 1);
    println!("{} {}", i2 + 1, j2 + 1);
}
