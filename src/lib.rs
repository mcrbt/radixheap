/*
 * radixheap - Radix heap data structure library
 * Copyright (C) 2019, 2020 Daniel Haase
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program.
 * If not, see <https://www.gnu.org/licenses/gpl-3.0.txt>.
 */

#![crate_type = "lib"]

pub mod radixheap {
	use std::cmp::Ordering;
	use std::fmt::Debug;
	use std::marker::PhantomData;

	#[derive(Clone, Debug)]
	pub struct Bucket<'a, V: 'a + Ord> {
		index: usize,
		top: Option<(u32, V)>,
		items: Vec<(u32, V)>,
		_phantom: PhantomData<&'a V>
	}

	#[derive(Clone, Debug)]
	pub struct RadixHeap<'a, V: 'a + Debug + Ord> {
		buckets: Vec<Bucket<'a, V>>,
		toplast: u32,
		length: usize
	}

	pub struct BucketIter<'a, V: 'a + Ord> {
		container: &'a Bucket<'a, V>,
		index: usize
	}

	pub struct IntoBucketIter<'a, V: 'a + Clone + Ord> {
		container: Bucket<'a, V>,
		index: usize
	}

	pub struct RadixBucketIter<'a, V: 'a + Debug + Ord> {
		container: &'a RadixHeap<'a, V>,
		index: usize
	}

	pub struct IntoRadixBucketIter<'a, V: 'a + Clone + Debug + Ord> {
		container: RadixHeap<'a, V>,
		index: usize
	}

	impl<'a, V: 'a + Ord> Bucket<'a, V> {
		fn length(&self) -> usize { self.items.len() }
		fn capacity(&self) -> usize { self.items.capacity() }
		fn empty(&self) -> bool { self.items.is_empty() }
		fn clear(&mut self) {
			self.items.clear();
			self.top = None
		}
		fn iter(&self) -> BucketIter<V> { BucketIter { container: self, index: 0 } }
	}

	impl<'a, V: 'a + Clone + Ord> Bucket<'a, V> {
		fn push(&mut self, key: u32, val: V) -> Result<(), &str> {
			// push key/value pair into bucket
			self.items.push((key, val.clone()));

			// update priority element of bucket
			if self.index == 0 { self.top = Some((key, val.clone())); } else {
				if let Some((k, _)) = self.top {
					if key < k { self.top = Some((key, val.clone())); }
				} else { self.top = Some((key, val.clone())); }
			}

			Ok(())
		}

		fn pop(&mut self) -> Option<(u32, V)> {
			let top = self.top.clone();
			self.top = self.iter().min_by_key(|(k, _)| k).cloned();

			if self.top.is_some() {
				self.items.remove(self.iter().position(|t| {
					if let Some((k, v)) = &top {
						t.0 == *k && (t.1).cmp(&v) == Ordering::Equal
					} else { false }
				}).unwrap());
			} else {
				assert!(top.is_none());
				eprintln!("cannot pop from empty bucket");
			}

			top
		}
	}

	impl<'a, V: 'a + Clone + Debug + Ord> RadixHeap<'a, V> {
		pub fn new(capacity: Option<usize>) -> RadixHeap<'a, V> {
			RadixHeap {
				buckets: (0..33).map(|i: usize| Bucket {
					index: i,
					top: None,
					items: Vec::with_capacity(capacity.unwrap_or(0)),
					_phantom: PhantomData {}
				}).collect(),
				toplast: std::u32::MIN,
				length: 0
			}
		}

		pub fn push(&mut self, key: u32, val: V) -> Result<(), &str> {
			// key smaller than key of last extracted element
			if key < self.toplast { Err("key too small") } else {
				// convention
				let bucket = if key == self.toplast { 0 } else { 32 - (key ^ self.toplast).leading_zeros() };

				// insert key/value pair into bucket
				self.buckets[bucket as usize].push(key, val.clone())?;
				self.length += 1;

				Ok(())
			}
		}

		pub fn pop(&mut self) -> Option<(u32, V)> {
			if self.empty() { return None; }

			let mut top: Option<(u32, V)> = None;
			let mut current;
			let mut index: usize = 0;

			#[allow(unused_mut)] // "bucket" needs to be mutable for "pop()"
			for mut bucket in &mut self.buckets {
				if !bucket.empty() {
					if bucket.index == 0 {
						self.length -= 1;
						return bucket.pop();
					} else {
						top = bucket.pop();

						// most important line for restructuring
						self.toplast = if let Some((k, _)) = top { k } else { return None; };
						index = bucket.index;

						// use first non-empty bucket for restructuring
						break;
					}
				}
			}

			current = self.buckets[index].clone();
			self.buckets[index] = Bucket {
				index,
				top: None,
				items: Vec::new(),
				_phantom: PhantomData
			};

			if !current.empty() {
				for _ in 0..current.length() {
					if let Some((k, v)) = current.pop() {
						// push uses updated bucket index for re-insertion:
						self.push(k, v.clone()).unwrap();
					} else { unreachable!() }
				}
			}

			// the original bucket must be empty after reorganizing the heap
			assert!(current.empty());
			self.length -= 1;
			top
		}

		pub fn peek(&self) -> Option<(u32, V)> {
			if self.empty() { return None; }

			for bucket in &self.buckets {
				if !bucket.empty() {
					if let Some((ref key, ref val)) = bucket.top {
						return Some((*key, val.clone())).clone();
					} else { return None; }
				}
			}

			None
		}

		pub fn length(&self) -> usize { self.length }

		pub fn capacity(&self) -> usize {
			self.buckets.iter().fold(0usize, |cap, b| { cap + b.capacity() })
		}

		pub fn empty(&self) -> bool { self.length == 0 }

		pub fn clear(&mut self) {
			self.buckets.iter_mut().all(|b| {
				b.clear();
				true
			});
			self.length = 0usize;
		}

		pub fn bucket_iter(&self) -> RadixBucketIter<V> {
			RadixBucketIter { container: self, index: 0 }
		}

		pub fn tuples(&self) -> Vec<(u32, V)> {
			self.bucket_iter().flat_map(|b| b.items.clone()).collect()
		}

		pub fn sorted_tuples(&self) -> Vec<(u32, V)> {
			#[allow(unused_mut)]
			let mut coll = &mut self.tuples();
			coll.as_mut_slice().sort_unstable_by(|a, b| { (a.0).cmp(&b.0) });
			coll.to_vec()
		}

		pub fn keys(&self) -> Vec<u32> {
			self.sorted_tuples().into_iter().map(|(k, _)| k).collect()
		}

		pub fn values(&self) -> Vec<V> {
			self.sorted_tuples().into_iter().map(|(_, v)| v.clone()).collect()
		}
	}

	impl<'a, V: 'a + Clone + Debug + Ord> Default for RadixHeap<'a, V> {
		fn default() -> RadixHeap<'a, V> { RadixHeap::new(None) }
	}

	impl<'a, V: 'a + Clone + Ord> Iterator for BucketIter<'a, V> {
		type Item = &'a (u32, V);

		fn next(&mut self) -> Option<Self::Item> {
			if self.index >= self.container.length() { None } else {
				self.index += 1;
				Some(&self.container.items[self.index - 1])
			}
		}
	}

	impl<'a, V: 'a + Clone + Ord> Iterator for IntoBucketIter<'a, V> {
		type Item = (u32, V);

		fn next(&mut self) -> Option<Self::Item> {
			if self.index >= self.container.length() { None } else {
				self.index += 1;
				Some(self.container.items[self.index - 1].clone())
			}
		}
	}

	impl<'a, V: 'a + Clone + Ord> IntoIterator for Bucket<'a, V> {
		type Item = (u32, V);
		type IntoIter = IntoBucketIter<'a, V>;

		fn into_iter(self) -> Self::IntoIter {
			IntoBucketIter { container: self, index: 0 }
		}
	}

	impl<'a, V: 'a + Clone + Debug + Ord> Iterator for RadixBucketIter<'a, V> {
		type Item = &'a Bucket<'a, V>;

		fn next(&mut self) -> Option<Self::Item> {
			if self.index >= self.container.buckets.len() { None } else {
				self.index += 1;
				Some(&self.container.buckets[self.index - 1])
			}
		}
	}

	impl<'a, V: 'a + Clone + Debug + Ord> Iterator for IntoRadixBucketIter<'a, V> {
		type Item = Bucket<'a, V>;

		fn next(&mut self) -> Option<Self::Item> {
			if self.index >= self.container.buckets.len() { None } else {
				self.index += 1;
				Some(self.container.buckets[self.index - 1].clone())
			}
		}
	}

	impl<'a, V: 'a + Clone + Debug + Ord> IntoIterator for RadixHeap<'a, V> {
		type Item = Bucket<'a, V>;
		type IntoIter = IntoRadixBucketIter<'a, V>;

		fn into_iter(self) -> Self::IntoIter {
			IntoRadixBucketIter { container: self, index: 0 }
		}
	}

	#[cfg(test)]
	mod test {
		use super::*;
		use rand::Rng;

		#[test]
		fn test_heap() {
			let mut heap = RadixHeap::default();
			assert!(heap.empty());
			assert_eq!(heap.length, 0);
			heap.push(7, 'a').unwrap();
			assert_eq!(heap.length, 1);
			heap.push(2, 'b').unwrap();
			heap.push(9, 'c').unwrap();

			assert_eq!(heap.peek(), Some((2, 'b')));
			assert_eq!(heap.pop(), Some((2, 'b')));
			assert_eq!(heap.toplast, 2);
			assert_eq!(heap.pop(), Some((7, 'a')));
			assert_eq!(heap.toplast, 7);
			assert_eq!(heap.pop(), Some((9, 'c')));
			assert_eq!(heap.toplast, 9);
			assert!(heap.empty());
		}

		#[test]
		fn test_pop() {
			let mut heap = RadixHeap::new(None);
			let mut rng = rand::thread_rng();
			let mut keys: Vec<u32> = Vec::with_capacity(100);

			for _ in 0..100 {
				let number: u32 = rng.gen();
				keys.push(number);
			}

			keys.sort_by(|a, b| b.cmp(a));

			assert_eq!(heap.capacity(), 0usize);

			for _ in 0..100 {
				let number: u32 = keys.pop().unwrap();
				heap.push(number, "").unwrap_or_else(|s| {
					assert!(false, "failed to push key {}: {}", number, s);
				});
				assert_eq!(heap.peek(), Some((number, "")));
				heap.pop();
			}

			assert!(heap.empty());
			heap.clear();
			assert!(heap.empty());
		}

		#[test]
		fn test_capacity() {
			let heap: RadixHeap<&str> = RadixHeap::new(Some(12usize));
			assert_eq!(heap.capacity(), 396 as usize);
			assert_eq!(heap.length(), 0usize);
			assert!(heap.empty());
		}

		#[test]
		#[allow(unused_must_use)]
		fn test_tuples() {
			let mut heap: RadixHeap<&str> = RadixHeap::new(Some(48usize));
			// let mut tupkeys: Vec<u32> = Vec::with_capacity(10usize);

			heap.push(289371, "library");
			heap.push(259, "radix");
			heap.push(98612, "heap");
			heap.push(34, "rust");

			assert_eq!(heap.tuples()
			               .into_iter()
				           .map(|(k, _)| k)
				           .collect::<Vec<u32>>(),
			           vec![34, 259, 98612, 289371]);
			assert_eq!(heap.sorted_tuples(), heap.tuples());
			assert_eq!(heap.values(), vec!["rust", "radix", "heap", "library"]);
			assert_eq!(heap.length(), 4usize);
			assert_eq!(heap.capacity(), 1584usize);
			assert!(!heap.empty());

			heap.clear();
			assert!(heap.empty());
			assert_eq!(heap.length(), 0usize);

			heap.push(15, "seven");
			heap.push(9, "four");
			heap.push(13, "thirteen");
			heap.push(12, "twelve");
			heap.push(10, "ten");
			heap.push(11, "eleven");
			heap.push(8, "eight");
			heap.push(17, "seventeen");
			heap.push(3, "three");

			assert_ne!(heap.tuples(), heap.sorted_tuples());
			assert_eq!(heap.tuples()
				           .into_iter()
				           .map(|(k, _)| k)
				           .collect::<Vec<u32>>(),
			           vec![3u32, 15, 9, 13, 12, 10, 11, 8, 17]);
			assert_eq!(heap.sorted_tuples()
						   .into_iter()
						   .map(|(k, _)| k)
				           .collect::<Vec<u32>>(),
					   vec![3u32, 8, 9, 10, 11, 12, 13, 15, 17]);
		}
	}
}
