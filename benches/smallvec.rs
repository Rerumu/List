use criterion::{
	black_box, criterion_group, criterion_main, measurement::WallTime, BatchSize, BenchmarkGroup,
	Criterion, Throughput,
};
use smallvec::SmallVec;

const ELEMENT_COUNT: usize = 128;

fn bench_push(group: &mut BenchmarkGroup<'_, WallTime>) {
	group.bench_function("push", |b| {
		b.iter_batched(
			SmallVec::<[usize; ELEMENT_COUNT]>::new,
			|mut small| {
				for index in black_box(0..ELEMENT_COUNT) {
					small.push(index);
				}

				small
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_insert(group: &mut BenchmarkGroup<'_, WallTime>) {
	group.bench_function("insert", |b| {
		b.iter_batched(
			SmallVec::<[usize; ELEMENT_COUNT]>::new,
			|mut small| {
				for index in black_box(0..ELEMENT_COUNT) {
					let len = black_box(0);

					small.insert(len, index);
				}

				small
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_insert_first(group: &mut BenchmarkGroup<'_, WallTime>) {
	group.bench_function("insert_first", |b| {
		b.iter_batched(
			SmallVec::<[usize; ELEMENT_COUNT]>::new,
			|mut small| {
				for index in black_box(0..ELEMENT_COUNT) {
					small.insert(0, index);
				}

				small
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_insert_last(group: &mut BenchmarkGroup<'_, WallTime>) {
	group.bench_function("insert_last", |b| {
		b.iter_batched(
			SmallVec::<[usize; ELEMENT_COUNT]>::new,
			|mut small| {
				for index in black_box(0..ELEMENT_COUNT) {
					let len = small.len();

					small.insert(len, index);
				}

				small
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_pop(group: &mut BenchmarkGroup<'_, WallTime>, small: &SmallVec<[usize; ELEMENT_COUNT]>) {
	group.bench_function("pop", |b| {
		b.iter_batched(
			|| small.clone(),
			|mut small| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let value = small.pop().expect("Failed to pop");

					black_box(value);
				}

				small
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_remove(
	group: &mut BenchmarkGroup<'_, WallTime>,
	small: &SmallVec<[usize; ELEMENT_COUNT]>,
) {
	group.bench_function("remove", |b| {
		b.iter_batched(
			|| small.clone(),
			|mut small| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let len = black_box(0);
					let value = small.remove(len);

					black_box(value);
				}

				small
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_remove_first(
	group: &mut BenchmarkGroup<'_, WallTime>,
	small: &SmallVec<[usize; ELEMENT_COUNT]>,
) {
	group.bench_function("remove_first", |b| {
		b.iter_batched(
			|| small.clone(),
			|mut small| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let value = small.remove(0);

					black_box(value);
				}

				small
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_remove_last(
	group: &mut BenchmarkGroup<'_, WallTime>,
	small: &SmallVec<[usize; ELEMENT_COUNT]>,
) {
	group.bench_function("remove_last", |b| {
		b.iter_batched(
			|| small.clone(),
			|mut small| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let len = small.len() - 1;
					let value = small.remove(len);

					black_box(value);
				}

				small
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_swap_remove(
	group: &mut BenchmarkGroup<'_, WallTime>,
	small: &SmallVec<[usize; ELEMENT_COUNT]>,
) {
	group.bench_function("swap_remove", |b| {
		b.iter_batched(
			|| small.clone(),
			|mut small| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let len = black_box(0);
					let value = small.swap_remove(len);

					black_box(value);
				}

				small
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_swap_remove_last(
	group: &mut BenchmarkGroup<'_, WallTime>,
	small: &SmallVec<[usize; ELEMENT_COUNT]>,
) {
	group.bench_function("swap_remove_last", |b| {
		b.iter_batched(
			|| small.clone(),
			|mut small| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let len = small.len() - 1;
					let value = small.swap_remove(len);

					black_box(value);
				}

				small
			},
			BatchSize::SmallInput,
		);
	});
}

fn benchmark_small_vec(c: &mut Criterion) {
	let mut group = c.benchmark_group("SmallVec");
	let elements = u64::try_from(ELEMENT_COUNT).unwrap();

	group.throughput(Throughput::Elements(elements));

	bench_push(&mut group);
	bench_insert(&mut group);
	bench_insert_first(&mut group);
	bench_insert_last(&mut group);

	let mut small = SmallVec::<[usize; ELEMENT_COUNT]>::new();

	small.extend(0..ELEMENT_COUNT);

	bench_pop(&mut group, &small);
	bench_remove(&mut group, &small);
	bench_remove_first(&mut group, &small);
	bench_remove_last(&mut group, &small);
	bench_swap_remove(&mut group, &small);
	bench_swap_remove_last(&mut group, &small);
}

criterion_group!(benches, benchmark_small_vec);
criterion_main!(benches);
