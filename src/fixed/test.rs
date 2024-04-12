use alloc::{boxed::Box, vec};

use crate::fixed::Fixed;

#[test]
fn test_push_elements() {
	let mut fixed = Fixed::<u64, 4>::new();

	assert_eq!(fixed.len(), 0);
	assert_eq!(fixed.try_push(1), Ok(()));

	assert_eq!(fixed.len(), 1);
	assert_eq!(fixed.try_push(2), Ok(()));

	assert_eq!(fixed.len(), 2);
	assert_eq!(fixed.try_push(3), Ok(()));

	assert_eq!(fixed.len(), 3);
	assert_eq!(fixed.try_push(4), Ok(()));

	assert_eq!(fixed.len(), 4);
	assert_eq!(fixed.try_push(5), Err(5));

	assert_eq!(fixed.as_slice(), &[1, 2, 3, 4]);
}

#[test]
fn test_insert_elements() {
	let mut fixed = Fixed::<u64, 4>::new();

	assert_eq!(fixed.len(), 0);
	assert_eq!(fixed.try_insert(0, 1), Ok(()));

	assert_eq!(fixed.len(), 1);
	assert_eq!(fixed.try_insert(0, 2), Ok(()));

	assert_eq!(fixed.len(), 2);
	assert_eq!(fixed.try_insert(1, 3), Ok(()));

	assert_eq!(fixed.len(), 3);
	assert_eq!(fixed.try_insert(1, 4), Ok(()));

	assert_eq!(fixed.len(), 4);
	assert_eq!(fixed.try_insert(3, 5), Err(5));

	assert_eq!(fixed.as_slice(), &[2, 4, 3, 1]);
}

#[test]
fn test_to_vec_reserve() {
	let mut fixed = Fixed::<u64, 4>::new();

	assert_eq!(fixed.len(), 0);
	assert_eq!(fixed.try_push(1), Ok(()));

	assert_eq!(fixed.len(), 1);
	assert_eq!(fixed.try_push(2), Ok(()));

	assert_eq!(fixed.len(), 2);
	assert_eq!(fixed.try_push(3), Ok(()));

	assert_eq!(fixed.len(), 3);
	assert_eq!(fixed.try_push(4), Ok(()));

	assert_eq!(fixed.to_vec_reserve(1), vec![1, 2, 3, 4]);
}

#[test]
fn test_pop_elements() {
	let mut fixed = Fixed::<u64, 4>::new();

	assert_eq!(fixed.len(), 0);
	assert_eq!(fixed.try_push(1), Ok(()));

	assert_eq!(fixed.len(), 1);
	assert_eq!(fixed.try_push(2), Ok(()));

	assert_eq!(fixed.len(), 2);
	assert_eq!(fixed.try_push(3), Ok(()));

	assert_eq!(fixed.len(), 3);
	assert_eq!(fixed.try_push(4), Ok(()));

	assert_eq!(fixed.try_pop(), Some(4));
	assert_eq!(fixed.try_pop(), Some(3));
	assert_eq!(fixed.try_pop(), Some(2));
	assert_eq!(fixed.try_pop(), Some(1));
	assert_eq!(fixed.try_pop(), None);
}

#[test]
fn test_remove_elements() {
	let mut fixed = Fixed::<u64, 4>::new();

	assert_eq!(fixed.len(), 0);
	assert_eq!(fixed.try_push(1), Ok(()));

	assert_eq!(fixed.len(), 1);
	assert_eq!(fixed.try_push(2), Ok(()));

	assert_eq!(fixed.len(), 2);
	assert_eq!(fixed.try_push(3), Ok(()));

	assert_eq!(fixed.len(), 3);
	assert_eq!(fixed.try_push(4), Ok(()));

	assert_eq!(fixed.try_remove(5), None);
	assert_eq!(fixed.try_remove(1), Some(2));
	assert_eq!(fixed.try_remove(1), Some(3));
	assert_eq!(fixed.try_remove(0), Some(1));
	assert_eq!(fixed.try_remove(0), Some(4));
	assert_eq!(fixed.try_remove(0), None);
}

#[test]
fn test_clear() {
	let mut fixed = Fixed::<Box<u64>, 128>::new();

	for index in 0..128 {
		assert_eq!(fixed.try_push(index.into()), Ok(()));
	}

	assert_eq!(fixed.len(), 128);

	fixed.clear();

	assert!(fixed.is_empty());
}

#[test]
fn test_into_iter() {
	let mut fixed = Fixed::<u64, 4>::new();

	assert_eq!(fixed.try_push(1), Ok(()));
	assert_eq!(fixed.try_push(2), Ok(()));
	assert_eq!(fixed.try_push(3), Ok(()));
	assert_eq!(fixed.try_push(4), Ok(()));

	let mut iter = fixed.into_iter();

	assert_eq!(iter.next(), Some(1));
	assert_eq!(iter.next_back(), Some(4));
	assert_eq!(iter.next(), Some(2));
	assert_eq!(iter.next_back(), Some(3));
	assert_eq!(iter.next(), None);
	assert_eq!(iter.next_back(), None);
}
