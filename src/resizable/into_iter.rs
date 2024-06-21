use core::iter::FusedIterator;

macro_rules! impl_mirrored {
	($item:expr, $list:pat => $apply:expr) => {
		match $item {
			Self::Fixed($list) => $apply,
			Self::Heap($list) => $apply,
		}
	};
}

/// An iterator that moves out of a list.
pub enum IntoIter<T, const N: usize> {
	Fixed(crate::fixed::IntoIter<T, N>),
	Heap(alloc::vec::IntoIter<T>),
}

impl<T, const N: usize> IntoIter<T, N> {
	/// Creates an iterator which returns no elements.
	#[inline]
	#[must_use]
	pub const fn empty() -> Self {
		Self::Fixed(crate::fixed::IntoIter::empty())
	}
}

// We have explicit implementations for each method that could be a bottleneck
// by having repeated calls to `next`. We don't include methods that rely on
// other methods that are specialized, such as `for_each` which uses `fold`.
impl<T, const N: usize> Iterator for IntoIter<T, N> {
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		impl_mirrored!(self, list => list.next())
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		impl_mirrored!(self, list => list.size_hint())
	}

	#[inline]
	fn count(self) -> usize {
		impl_mirrored!(self, list => list.count())
	}

	#[inline]
	fn last(self) -> Option<Self::Item> {
		impl_mirrored!(self, list => list.last())
	}

	#[inline]
	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		impl_mirrored!(self, list => list.nth(n))
	}

	#[inline]
	fn collect<B: FromIterator<Self::Item>>(self) -> B {
		impl_mirrored!(self, list => list.collect())
	}

	#[inline]
	fn fold<B, F>(self, init: B, f: F) -> B
	where
		F: FnMut(B, Self::Item) -> B,
	{
		impl_mirrored!(self, list => list.fold(init, f))
	}

	#[inline]
	fn reduce<F>(self, f: F) -> Option<Self::Item>
	where
		F: FnMut(Self::Item, Self::Item) -> Self::Item,
	{
		impl_mirrored!(self, list => list.reduce(f))
	}

	#[inline]
	fn all<F>(&mut self, f: F) -> bool
	where
		F: FnMut(Self::Item) -> bool,
	{
		impl_mirrored!(self, list => list.all(f))
	}

	#[inline]
	fn any<F>(&mut self, f: F) -> bool
	where
		F: FnMut(Self::Item) -> bool,
	{
		impl_mirrored!(self, list => list.any(f))
	}

	#[inline]
	fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
	where
		P: FnMut(&Self::Item) -> bool,
	{
		impl_mirrored!(self, list => list.find(predicate))
	}

	#[inline]
	fn find_map<B, F>(&mut self, f: F) -> Option<B>
	where
		F: FnMut(Self::Item) -> Option<B>,
	{
		impl_mirrored!(self, list => list.find_map(f))
	}

	#[inline]
	fn position<P>(&mut self, predicate: P) -> Option<usize>
	where
		P: FnMut(Self::Item) -> bool,
	{
		impl_mirrored!(self, list => list.position(predicate))
	}

	#[inline]
	fn rposition<P>(&mut self, predicate: P) -> Option<usize>
	where
		P: FnMut(Self::Item) -> bool,
	{
		impl_mirrored!(self, list => list.rposition(predicate))
	}
}

impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N> {
	#[inline]
	fn len(&self) -> usize {
		impl_mirrored!(self, list => list.len())
	}
}

impl<T, const N: usize> DoubleEndedIterator for IntoIter<T, N> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		impl_mirrored!(self, list => list.next_back())
	}

	#[inline]
	fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
		impl_mirrored!(self, list => list.nth_back(n))
	}

	fn rfold<B, F>(self, init: B, f: F) -> B
	where
		F: FnMut(B, Self::Item) -> B,
	{
		impl_mirrored!(self, list => list.rfold(init, f))
	}

	fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
	where
		P: FnMut(&Self::Item) -> bool,
	{
		impl_mirrored!(self, list => list.rfind(predicate))
	}
}

impl<T, const N: usize> FusedIterator for IntoIter<T, N> {}
