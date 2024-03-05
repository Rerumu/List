use std::ops::{Deref, DerefMut};

use crate::fixed::Fixed;

use super::into_iter::IntoIter;

macro_rules! impl_mirrored {
	($item:expr, $list:pat => $apply:expr) => {
		match $item {
			Self::Fixed($list) => $apply,
			Self::Heap($list) => $apply,
		}
	};
}

/// A contiguous resizable list of elements of type `T`.
pub enum Resizable<T, const N: usize> {
	Fixed(Fixed<T, N>),
	Heap(Vec<T>),
}

impl<T, const N: usize> Resizable<T, N> {
	/// Constructs a new, empty `Resizable<T, N>`.
	#[inline]
	#[must_use]
	pub const fn new() -> Self {
		Self::Fixed(Fixed::new())
	}

	/// Returns the total number of elements the list can hold.
	#[inline]
	#[must_use]
	pub fn with_capacity(capacity: usize) -> Self {
		if capacity <= N {
			Self::Fixed(Fixed::new())
		} else {
			Self::Heap(Vec::with_capacity(capacity))
		}
	}

	/// Reserves capacity for at least `additional` more elements to be inserted
	/// in the given list.
	#[inline]
	pub fn reserve(&mut self, additional: usize) {
		match self {
			Self::Fixed(list) => {
				if list.capacity() - list.len() < additional {
					let list = std::mem::take(list).to_vec_reserve(additional);

					*self = Self::Heap(list);
				}
			}
			Self::Heap(list) => list.reserve(additional),
		}
	}

	/// Reserves the minimum capacity for at least `additional` more elements to
	/// be inserted in the given list.
	#[inline]
	pub fn reserve_exact(&mut self, additional: usize) {
		match self {
			Self::Fixed(list) => {
				if list.capacity() - list.len() < additional {
					let list = std::mem::take(list).to_vec_reserve(additional);

					*self = Self::Heap(list);
				}
			}
			Self::Heap(list) => list.reserve_exact(additional),
		}
	}

	/// Returns the total number of elements the list can hold.
	#[inline]
	#[must_use]
	pub fn capacity(&self) -> usize {
		impl_mirrored!(self, list => list.capacity())
	}

	/// Returns the number of elements in the list.
	#[inline]
	#[must_use]
	pub fn len(&self) -> usize {
		impl_mirrored!(self, list => list.len())
	}

	/// Returns `true` if the list contains no elements.
	#[inline]
	#[must_use]
	pub fn is_empty(&self) -> bool {
		impl_mirrored!(self, list => list.is_empty())
	}

	/// Returns a pointer to the first element of the list.
	#[inline]
	#[must_use]
	pub fn as_ptr(&self) -> *const T {
		impl_mirrored!(self, list => list.as_ptr())
	}

	/// Returns a mutable pointer to the first element of the list.
	#[inline]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut T {
		impl_mirrored!(self, list => list.as_mut_ptr())
	}

	/// Extracts a slice containing the entire list.
	#[inline]
	#[must_use]
	pub fn as_slice(&self) -> &[T] {
		impl_mirrored!(self, list => list.as_slice())
	}

	/// Extracts a mutable slice of the entire list.
	#[inline]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [T] {
		impl_mirrored!(self, list => list.as_mut_slice())
	}

	/// Clears the list, removing all values.
	#[inline]
	pub fn clear(&mut self) {
		impl_mirrored!(self, list => list.clear());
	}

	/// Inserts an element at position `index` within the list, shifting all
	/// elements after it to the right.
	#[inline]
	pub fn insert(&mut self, index: usize, value: T) {
		#[cold]
		fn heap_insert<T, const N: usize>(
			list: &mut Fixed<T, N>,
			index: usize,
			value: T,
		) -> Resizable<T, N> {
			let len = list.len();
			let mut heap = std::mem::take(list).to_vec_reserve(len);

			heap.insert(index, value);

			Resizable::Heap(heap)
		}

		match self {
			Self::Fixed(list) => {
				if let Err(value) = list.try_insert(index, value) {
					*self = heap_insert(list, index, value);
				}
			}
			Self::Heap(list) => list.insert(index, value),
		}
	}

	/// Appends an element to the back of the list.
	#[inline]
	pub fn push(&mut self, value: T) {
		#[cold]
		fn heap_push<T, const N: usize>(list: &mut Fixed<T, N>, value: T) -> Resizable<T, N> {
			let len = list.len();
			let mut heap = std::mem::take(list).to_vec_reserve(len);

			heap.push(value);

			Resizable::Heap(heap)
		}

		match self {
			Self::Fixed(list) => {
				if let Err(value) = list.try_push(value) {
					*self = heap_push(list, value);
				}
			}
			Self::Heap(list) => list.push(value),
		}
	}

	/// Removes and returns the element at position `index` within the list,
	/// shifting all elements after it to the left.
	#[inline]
	pub fn remove(&mut self, index: usize) -> T {
		#[cold]
		fn assert_failed(index: usize, len: usize) -> ! {
			panic!("removal index (is {index}) should be < len (is {len})");
		}

		match self {
			Self::Fixed(list) => list
				.try_remove(index)
				.unwrap_or_else(|| assert_failed(index, list.len())),

			Self::Heap(list) => list.remove(index),
		}
	}

	/// Removes the last element from a list and returns it, or [`None`] if it
	/// is empty.
	#[inline]
	pub fn pop(&mut self) -> Option<T> {
		match self {
			Self::Fixed(list) => list.try_pop(),
			Self::Heap(list) => list.pop(),
		}
	}

	/// Removes an element from the list and returns it.
	///
	/// The removed element is replaced by the last element of the list.
	#[inline]
	pub fn swap_remove(&mut self, index: usize) -> T {
		#[cold]
		fn assert_failed(index: usize, len: usize) -> ! {
			panic!("swap_remove index (is {index}) should be < len (is {len})");
		}

		match self {
			Self::Fixed(list) => list
				.try_swap_remove(index)
				.unwrap_or_else(|| assert_failed(index, list.len())),

			Self::Heap(list) => list.swap_remove(index),
		}
	}
}

impl<T, const N: usize> IntoIterator for Resizable<T, N> {
	type IntoIter = IntoIter<T, N>;
	type Item = T;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		match self {
			Self::Fixed(list) => IntoIter::Fixed(list.into_iter()),
			Self::Heap(list) => IntoIter::Heap(list.into_iter()),
		}
	}
}

impl<T, const N: usize> Default for Resizable<T, N> {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

impl<T, const N: usize> Extend<T> for Resizable<T, N> {
	#[inline]
	fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
		let iter = iter.into_iter();

		self.reserve(iter.size_hint().0);

		iter.for_each(|value| self.push(value));
	}
}

impl<T: Clone, const N: usize> Clone for Resizable<T, N> {
	#[inline]
	fn clone(&self) -> Self {
		if self.len() < N {
			let mut list = Fixed::new();

			list.extend(self.as_slice().iter().cloned());

			Self::Fixed(list)
		} else {
			Self::Heap(self.as_slice().to_vec())
		}
	}

	#[inline]
	fn clone_from(&mut self, source: &Self) {
		self.clear();

		self.extend(source.iter().cloned());
	}
}

impl<T, const N: usize> Deref for Resizable<T, N> {
	type Target = [T];

	#[inline]
	fn deref(&self) -> &Self::Target {
		self.as_slice()
	}
}

impl<T, const N: usize> DerefMut for Resizable<T, N> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_slice()
	}
}

impl<T: std::fmt::Debug, const N: usize> std::fmt::Debug for Resizable<T, N> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.as_slice().fmt(f)
	}
}
