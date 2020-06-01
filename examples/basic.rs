/*
 * radixheap - Radix heap data structure library
 * Copyright (C) 2019, 2020 Daniel Haase
 *
 * File: basic.rs
 * Author: Daniel Haase
 *
 * This file is part of radixheap.
 *
 * radixheap is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * radixheap is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with radixheap.
 * If not, see <https://www.gnu.org/licenses/lgpl-3.0.txt>.
 */

use radixheap::radixheap::RadixHeap;

fn main() {
    let mut heap: RadixHeap<&str> = RadixHeap::new(Some(8));

    heap.push(18, "of").unwrap();
    heap.push(93, "rust").unwrap();
    heap.push(7, "amazing").unwrap();
    heap.push(1, "hello").unwrap();
    heap.push(13, "world").unwrap();
    heap.push(211, "development").unwrap();

    assert_eq!(heap.length(), 6);
    assert!(heap.capacity() >= heap.length());
    assert_eq!(heap.capacity(), 264);
    assert!(!heap.empty());
    assert_eq!(heap.peek(), Some((1, "hello")));
    assert_eq!(heap.tuples().first(), Some(&(1, "hello")));
    assert_eq!(heap.keys(), vec![1, 7, 13, 18, 93, 211]);
    assert_eq!(heap.values().join::<&str>(" "),
               "hello amazing world of rust development");
    println!("{}", heap.values().join::<&str>(" "));

    heap.pop();
    assert_eq!(heap.peek(), Some((7, "amazing")));

    for _ in 0..(heap.length() - 2) {
        heap.pop();
        assert!(!heap.empty());
    }

    assert_eq!(heap.values().join::<&str>(" "), "rust development");

    heap.clear();
    assert_eq!(heap.length(), 0);
    assert!(heap.empty());
}
