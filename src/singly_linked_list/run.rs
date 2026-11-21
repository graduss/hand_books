use std::io::{self, BufRead};

use crate::singly_linked_list::SinglyLinkedList;

pub fn run() {
    let mut list = SinglyLinkedList::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let args: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        match args.as_slice() {
            [_] => continue,
            [1, ind, value] => list.insert((*ind).try_into().unwrap(), *value).unwrap(),
            [2, ind] => println!("{}", list.get((*ind - 1).try_into().unwrap()).unwrap()),
            [3, ind] => {
                list.remove((*ind - 1).try_into().unwrap()).unwrap();
            }
            _ => panic!("Ivalid input"),
        }
    }
}
