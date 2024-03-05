use criterion::{
	black_box, criterion_group, criterion_main, measurement::WallTime, BatchSize, BenchmarkGroup,
	Criterion, Throughput,
};
use list::fixed::Fixed;

const ELEMENT_COUNT: usize = 128;

fn bench_try_push(group: &mut BenchmarkGroup<'_, WallTime>) {
	group.bench_function("try_push", |b| {
		b.iter_batched(
			Fixed::<usize, ELEMENT_COUNT>::new,
			|mut fixed| {
				for index in black_box(0..ELEMENT_COUNT) {
					fixed.try_push(index).expect("Failed to push");
				}

				fixed
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_try_insert(group: &mut BenchmarkGroup<'_, WallTime>) {
	group.bench_function("try_insert", |b| {
		b.iter_batched(
			Fixed::<usize, ELEMENT_COUNT>::new,
			|mut fixed| {
				for index in black_box(0..ELEMENT_COUNT) {
					let len = black_box(0);

					fixed.try_insert(len, index).expect("Failed to insert");
				}

				fixed
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_try_insert_first(group: &mut BenchmarkGroup<'_, WallTime>) {
	group.bench_function("try_insert_first", |b| {
		b.iter_batched(
			Fixed::<usize, ELEMENT_COUNT>::new,
			|mut fixed| {
				for index in black_box(0..ELEMENT_COUNT) {
					fixed.try_insert(0, index).expect("Failed to insert");
				}

				fixed
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_try_insert_last(group: &mut BenchmarkGroup<'_, WallTime>) {
	group.bench_function("try_insert_last", |b| {
		b.iter_batched(
			Fixed::<usize, ELEMENT_COUNT>::new,
			|mut fixed| {
				for index in black_box(0..ELEMENT_COUNT) {
					let len = fixed.len();

					fixed.try_insert(len, index).expect("Failed to insert");
				}

				fixed
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_try_pop(group: &mut BenchmarkGroup<'_, WallTime>, fixed: &Fixed<usize, ELEMENT_COUNT>) {
	group.bench_function("try_pop", |b| {
		b.iter_batched(
			|| fixed.clone(),
			|mut fixed| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let value = fixed.try_pop().expect("Failed to pop");

					black_box(value);
				}

				fixed
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_try_remove(group: &mut BenchmarkGroup<'_, WallTime>, fixed: &Fixed<usize, ELEMENT_COUNT>) {
	group.bench_function("try_remove", |b| {
		b.iter_batched(
			|| fixed.clone(),
			|mut fixed| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let len = black_box(0);
					let value = fixed.try_remove(len).expect("Failed to remove");

					black_box(value);
				}

				fixed
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_try_remove_first(
	group: &mut BenchmarkGroup<'_, WallTime>,
	fixed: &Fixed<usize, ELEMENT_COUNT>,
) {
	group.bench_function("try_remove_first", |b| {
		b.iter_batched(
			|| fixed.clone(),
			|mut fixed| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let value = fixed.try_remove(0).expect("Failed to remove");

					black_box(value);
				}

				fixed
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_try_remove_last(
	group: &mut BenchmarkGroup<'_, WallTime>,
	fixed: &Fixed<usize, ELEMENT_COUNT>,
) {
	group.bench_function("try_remove_last", |b| {
		b.iter_batched(
			|| fixed.clone(),
			|mut fixed| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let len = fixed.len() - 1;
					let value = fixed.try_remove(len).expect("Failed to remove");

					black_box(value);
				}

				fixed
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_try_swap_remove(
	group: &mut BenchmarkGroup<'_, WallTime>,
	fixed: &Fixed<usize, ELEMENT_COUNT>,
) {
	group.bench_function("try_swap_remove", |b| {
		b.iter_batched(
			|| fixed.clone(),
			|mut fixed| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let len = black_box(0);
					let value = fixed.try_swap_remove(len).expect("Failed to swap remove");

					black_box(value);
				}

				fixed
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_try_swap_remove_last(
	group: &mut BenchmarkGroup<'_, WallTime>,
	fixed: &Fixed<usize, ELEMENT_COUNT>,
) {
	group.bench_function("try_swap_remove_last", |b| {
		b.iter_batched(
			|| fixed.clone(),
			|mut fixed| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let len = fixed.len() - 1;
					let value = fixed.try_swap_remove(len).expect("Failed to swap remove");

					black_box(value);
				}

				fixed
			},
			BatchSize::SmallInput,
		);
	});
}

fn benchmark_fixed(c: &mut Criterion) {
	let mut group = c.benchmark_group("Fixed");
	let elements = u64::try_from(ELEMENT_COUNT).unwrap();

	group.throughput(Throughput::Elements(elements));

	bench_try_push(&mut group);
	bench_try_insert(&mut group);
	bench_try_insert_first(&mut group);
	bench_try_insert_last(&mut group);

	let mut fixed = Fixed::<usize, ELEMENT_COUNT>::new();

	fixed.extend(0..ELEMENT_COUNT);

	bench_try_pop(&mut group, &fixed);
	bench_try_remove(&mut group, &fixed);
	bench_try_remove_first(&mut group, &fixed);
	bench_try_remove_last(&mut group, &fixed);
	bench_try_swap_remove(&mut group, &fixed);
	bench_try_swap_remove_last(&mut group, &fixed);
}

criterion_group!(benches, benchmark_fixed);
criterion_main!(benches);
