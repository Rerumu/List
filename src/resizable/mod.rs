#[cfg(test)]
mod test;

mod collection;
mod into_iter;

pub use collection::Resizable;
pub use into_iter::IntoIter;

#[macro_export]
macro_rules! resizable {
	() => (
		$crate::resizable::Resizable::new()
	);
	($element:expr; $count:expr) => ({
		let iter = ::core::iter::repeat_n($element, $count);

		$crate::resizable::Resizable::from_iter(iter)
	});
	($($element:expr),+ $(,)?) => ({
		let iter = [$($element),+];

		$crate::resizable::Resizable::from_iter(iter)
	});
}
