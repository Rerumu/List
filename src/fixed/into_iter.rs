use core::{iter::FusedIterator, mem::MaybeUninit};

/// A by-value short array iterator.
/// Needed until [`std::array::IntoIter::new_unchecked`] is stabilized.
pub struct IntoIter<T, const N: usize> {
	start: u8,
	end: u8,
	inner: [MaybeUninit<T>; N],
}

impl<T, const N: usize> IntoIter<T, N> {
	/// Creates an iterator over the elements in a partially-initialized buffer.
	///
	/// # Panics
	///
	/// Panics if the capacity of the list does not fit within a `u8`.
	///
	/// # Safety
	///
	/// The elements in the buffer must be initialized in the range `start..end`.
	#[inline]
	#[must_use]
	pub const unsafe fn new_unchecked(inner: [MaybeUninit<T>; N], start: u8, end: u8) -> Self {
		debug_assert!(start <= end, "`start` must be less than or equal to `end`");
		debug_assert!(end as usize <= N, "`end` must be less than or equal to `N`");

		assert!(N <= u8::MAX as usize, "`Fixed` capacity exceeds `u8`");

		Self { start, end, inner }
	}

	/// Creates an iterator which returns no elements.
	///
	/// # Panics
	///
	/// Panics if the capacity of the list does not fit within a `u8`.
	#[inline]
	#[must_use]
	pub const fn empty() -> Self {
		let inner = unsafe { MaybeUninit::uninit().assume_init() };

		unsafe { Self::new_unchecked(inner, 0, 0) }
	}
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		if self.start >= self.end {
			None
		} else {
			let start = usize::from(self.start);

			self.start += 1;

			Some(unsafe { self.inner.get_unchecked(start).assume_init_read() })
		}
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = usize::from(self.end - self.start);

		if len > N {
			// SAFETY: `len` is always a valid length for the array.
			unsafe { core::hint::unreachable_unchecked() };
		}

		(len, Some(len))
	}

	#[inline]
	fn count(self) -> usize {
		self.len()
	}

	#[inline]
	fn last(mut self) -> Option<Self::Item> {
		self.next_back()
	}
}

impl<T, const N: usize> DoubleEndedIterator for IntoIter<T, N> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.start >= self.end {
			None
		} else {
			self.end -= 1;

			let end = usize::from(self.end);

			Some(unsafe { self.inner.get_unchecked(end).assume_init_read() })
		}
	}
}

impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N> {}

impl<T, const N: usize> FusedIterator for IntoIter<T, N> {}

impl<T, const N: usize> Drop for IntoIter<T, N> {
	#[inline]
	fn drop(&mut self) {
		self.for_each(drop);
	}
}
