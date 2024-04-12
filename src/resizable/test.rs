use alloc::boxed::Box;

use crate::resizable::Resizable;

#[test]
fn test_push_elements() {
	let mut resizable = Resizable::<u64, 2>::new();

	assert_eq!(resizable.len(), 0);
	resizable.push(1);

	assert_eq!(resizable.len(), 1);
	resizable.push(2);

	assert_eq!(resizable.len(), 2);
	resizable.push(3);

	assert_eq!(resizable.len(), 3);
	resizable.push(4);

	assert_eq!(resizable.as_slice(), &[1, 2, 3, 4]);
}

#[test]
fn test_insert_elements() {
	let mut resizable = Resizable::<u64, 2>::new();

	assert_eq!(resizable.len(), 0);
	resizable.insert(0, 1);

	assert_eq!(resizable.len(), 1);
	resizable.insert(0, 2);

	assert_eq!(resizable.len(), 2);
	resizable.insert(1, 3);

	assert_eq!(resizable.len(), 3);
	resizable.insert(1, 4);

	assert_eq!(resizable.as_slice(), &[2, 4, 3, 1]);
}

#[test]
fn test_pop_elements() {
	let mut resizable = Resizable::<u64, 2>::new();

	resizable.push(1);
	resizable.push(2);
	resizable.push(3);
	resizable.push(4);

	assert_eq!(resizable.pop(), Some(4));
	assert_eq!(resizable.pop(), Some(3));
	assert_eq!(resizable.pop(), Some(2));
	assert_eq!(resizable.pop(), Some(1));
	assert_eq!(resizable.pop(), None);
}

#[test]
fn test_remove_elements() {
	let mut resizable = Resizable::<u64, 2>::new();

	resizable.push(1);
	resizable.push(2);
	resizable.push(3);
	resizable.push(4);

	assert_eq!(resizable.remove(1), 2);
	assert_eq!(resizable.remove(1), 3);
	assert_eq!(resizable.remove(0), 1);
	assert_eq!(resizable.remove(0), 4);
}

#[test]
fn test_clear() {
	let mut resizable = Resizable::<Box<u64>, 24>::new();

	for index in 0..128 {
		resizable.push(index.into());
	}

	assert_eq!(resizable.len(), 128);

	resizable.clear();

	assert!(resizable.is_empty());
}

#[test]
fn test_into_iter() {
	let mut resizable = Resizable::<u64, 2>::new();

	resizable.push(1);
	resizable.push(2);
	resizable.push(3);
	resizable.push(4);

	let mut iter = resizable.into_iter();

	assert_eq!(iter.next(), Some(1));
	assert_eq!(iter.next_back(), Some(4));
	assert_eq!(iter.next(), Some(2));
	assert_eq!(iter.next_back(), Some(3));
	assert_eq!(iter.next(), None);
	assert_eq!(iter.next_back(), None);
}
