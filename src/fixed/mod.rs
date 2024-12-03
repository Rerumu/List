#[cfg(test)]
mod test;

mod collection;
mod into_iter;

pub use collection::Fixed;
pub use into_iter::IntoIter;

#[macro_export]
macro_rules! fixed {
	() => (
		$crate::fixed::Fixed::new()
	);
	($element:expr; $count:expr) => ({
		let iter = ::core::iter::repeat_n($element, $count);

		$crate::fixed::Fixed::from_iter(iter)
	});
	($($element:expr),+ $(,)?) => ({
		let array = [$($element),+];

		$crate::fixed::Fixed::from_iter(array)
	});
}
