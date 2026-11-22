use std::fmt::Write as _;
use std::io::{self, Read, Write};

pub fn run() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let q = it.next().unwrap();

    const MAX: usize = 100_000;
    let mut first = vec![-1i32; MAX + 1];
    for i in 1..=n {
        let a = it.next().unwrap();
        if first[a] == -1 {
            first[a] = i as i32;
        }
    }

    let mut out = String::with_capacity(q * 7);
    for _ in 0..q {
        let x = it.next().unwrap();
        writeln!(out, "{}", first[x]).unwrap();
    }

    io::stdout().write_all(out.as_bytes()).unwrap();
}
