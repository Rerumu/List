use std::mem::MaybeUninit;

use super::into_iter::IntoIter;

/// A contiguous fixed-size list of elements of type `T`.
pub struct Fixed<T, const N: usize> {
	inner: [MaybeUninit<T>; N],
	len: u8,
}

impl<T, const N: usize> Fixed<T, N> {
	/// Constructs a new, empty `Fixed<T, N>`.
	///
	/// # Panics
	///
	/// Panics if the capacity of the list does not fit within a `u8`.
	#[inline]
	#[must_use]
	pub const fn new() -> Self {
		assert!(N <= u8::MAX as usize, "`Fixed` capacity exceeds `u8`");

		Self {
			inner: unsafe { MaybeUninit::uninit().assume_init() },
			len: 0,
		}
	}

	/// Returns the total number of elements the list can hold.
	#[inline]
	#[must_use]
	pub const fn capacity(&self) -> usize {
		self.inner.len()
	}

	/// Returns the number of elements in the list.
	#[inline]
	#[must_use]
	pub const fn len(&self) -> usize {
		let len = self.len as usize;

		if len > self.capacity() {
			// SAFETY: `len` is always a valid length for the array.
			unsafe { std::hint::unreachable_unchecked() };
		}

		len
	}

	/// Returns `true` if the list contains no elements.
	#[inline]
	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Returns a pointer to the first element of the list.
	#[inline]
	#[must_use]
	pub const fn as_ptr(&self) -> *const T {
		self.inner.as_ptr().cast()
	}

	/// Returns a mutable pointer to the first element of the list.
	#[inline]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut T {
		self.inner.as_mut_ptr().cast()
	}

	/// Extracts a slice containing the entire list.
	#[inline]
	#[must_use]
	pub const fn as_slice(&self) -> &[T] {
		// SAFETY: `self.len` is always a valid length for the array.
		unsafe { std::slice::from_raw_parts(self.as_ptr(), self.len()) }
	}

	/// Extracts a mutable slice of the entire list.
	#[inline]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [T] {
		// SAFETY: `self.len` is always a valid length for the array.
		unsafe { std::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
	}

	/// Clears the list, removing all values.
	#[inline]
	pub fn clear(&mut self) {
		let slice: *mut [T] = std::ptr::slice_from_raw_parts_mut(self.as_mut_ptr(), self.len());

		self.len = 0;

		// SAFETY: `slice` is a valid slice of length `self.len`.
		unsafe { slice.drop_in_place() };
	}

	/// Inserts an element at position `index` within the list, shifting all
	/// elements after it to the right.
	///
	/// # Errors
	///
	/// Returns `Err` with the inserted value if the list is at capacity or the
	/// index is out of bounds.
	#[inline]
	pub fn try_insert(&mut self, index: usize, value: T) -> Result<(), T> {
		let len = self.len();

		if len < self.capacity() && index <= len {
			// SAFETY: `index` is less than or equal to `len`, and `len + 1` is a valid length for the array.
			let start = unsafe { self.inner.as_mut_ptr().add(index) };

			unsafe { start.add(1).copy_from(start, len - index) };
			unsafe { start.cast::<T>().write(value) };

			self.len += 1;

			Ok(())
		} else {
			Err(value)
		}
	}

	/// Appends an element to the back of the list.
	///
	/// # Errors
	///
	/// Returns `Err` with the pushed value if the list is at capacity.
	#[inline]
	pub fn try_push(&mut self, value: T) -> Result<(), T> {
		let len = self.len();

		self.try_insert(len, value)
	}

	/// Removes and returns the element at position `index` within the list,
	/// shifting all elements after it to the left, or returns [`None`] if the
	/// index is out of bounds.
	#[inline]
	pub fn try_remove(&mut self, index: usize) -> Option<T> {
		let len = self.len();

		if index < len {
			// SAFETY: `index` is less than `len`, and `len` is a valid length for the array.
			let start = unsafe { self.inner.as_mut_ptr().add(index) };
			let value = unsafe { start.cast::<T>().read() };

			unsafe { start.add(1).copy_to(start, len - index - 1) };

			self.len -= 1;

			Some(value)
		} else {
			None
		}
	}

	/// Removes the last element from a list and returns it, or [`None`] if it
	/// is empty.
	#[inline]
	pub fn try_pop(&mut self) -> Option<T> {
		let last = self.len().checked_sub(1)?;

		self.try_remove(last)
	}

	/// Removes an element from the list and returns it.
	///
	/// The removed element is replaced by the last element of the list.
	#[inline]
	pub fn try_swap_remove(&mut self, index: usize) -> Option<T> {
		let last = self.len().checked_sub(1)?;

		if index <= last {
			let start = self.inner.as_mut_ptr();

			unsafe { std::ptr::swap(start.add(index), start.add(last)) };

			self.try_remove(last)
		} else {
			None
		}
	}

	/// Converts the list into a `Vec<T>`, consuming the list.
	#[inline]
	pub fn to_vec_reserve(self, additional: usize) -> Vec<T> {
		let mut vec = Vec::with_capacity(self.len() + additional);

		vec.extend(self);

		vec
	}

	/// Creates a `Fixed<T, N>` directly from an array of `MaybeUninit<T>` and a length.
	///
	/// # Safety
	///
	/// `len` must be a valid length for the initialized part of the array.
	#[inline]
	pub const unsafe fn from_raw_parts(inner: [MaybeUninit<T>; N], len: u8) -> Self {
		Self { inner, len }
	}

	/// Decomposes a `Fixed<T, N>` into its raw components.
	#[inline]
	pub fn into_raw_parts(mut self) -> ([MaybeUninit<T>; N], u8) {
		let inner = std::mem::replace(&mut self.inner, unsafe {
			MaybeUninit::uninit().assume_init()
		});
		let len = std::mem::take(&mut self.len);

		(inner, len)
	}
}

impl<T, const N: usize> IntoIterator for Fixed<T, N> {
	type Item = T;
	type IntoIter = IntoIter<T, N>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		let (inner, len) = self.into_raw_parts();

		// SAFETY: `len` is always a valid length for the array.
		unsafe { Self::IntoIter::new_unchecked(inner, 0, len) }
	}
}

impl<T, const N: usize> Default for Fixed<T, N> {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

impl<T, const N: usize> Extend<T> for Fixed<T, N> {
	#[inline]
	fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
		iter.into_iter()
			.try_for_each(|value| self.try_push(value))
			.unwrap_or_else(|_| panic!("`Fixed` capacity exceeded"));
	}
}

impl<T: Clone, const N: usize> Clone for Fixed<T, N> {
	#[inline]
	fn clone(&self) -> Self {
		let mut result = Self::new();

		result.extend(self.iter().cloned());

		result
	}
}

impl<T, const N: usize> Drop for Fixed<T, N> {
	#[inline]
	fn drop(&mut self) {
		self.clear();
	}
}

impl<T, const N: usize> std::ops::Deref for Fixed<T, N> {
	type Target = [T];

	#[inline]
	fn deref(&self) -> &Self::Target {
		self.as_slice()
	}
}

impl<T, const N: usize> std::ops::DerefMut for Fixed<T, N> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_slice()
	}
}

impl<T: std::fmt::Debug, const N: usize> std::fmt::Debug for Fixed<T, N> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.as_slice().fmt(f)
	}
}