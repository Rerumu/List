use criterion::{
	black_box, criterion_group, criterion_main, measurement::WallTime, BatchSize, BenchmarkGroup,
	Criterion, Throughput,
};
use tinyvec::TinyVec;

const ELEMENT_COUNT: usize = 128;

fn bench_push(group: &mut BenchmarkGroup<'_, WallTime>) {
	group.bench_function("push", |b| {
		b.iter_batched(
			TinyVec::<[usize; ELEMENT_COUNT]>::new,
			|mut tiny| {
				for index in black_box(0..ELEMENT_COUNT) {
					tiny.push(index);
				}

				tiny
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_insert(group: &mut BenchmarkGroup<'_, WallTime>) {
	group.bench_function("insert", |b| {
		b.iter_batched(
			TinyVec::<[usize; ELEMENT_COUNT]>::new,
			|mut tiny| {
				for index in black_box(0..ELEMENT_COUNT) {
					let len = black_box(0);

					tiny.insert(len, index);
				}

				tiny
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_insert_first(group: &mut BenchmarkGroup<'_, WallTime>) {
	group.bench_function("insert_first", |b| {
		b.iter_batched(
			TinyVec::<[usize; ELEMENT_COUNT]>::new,
			|mut tiny| {
				for index in black_box(0..ELEMENT_COUNT) {
					tiny.insert(0, index);
				}

				tiny
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_insert_last(group: &mut BenchmarkGroup<'_, WallTime>) {
	group.bench_function("insert_last", |b| {
		b.iter_batched(
			TinyVec::<[usize; ELEMENT_COUNT]>::new,
			|mut tiny| {
				for index in black_box(0..ELEMENT_COUNT) {
					let len = tiny.len();

					tiny.insert(len, index);
				}

				tiny
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_pop(group: &mut BenchmarkGroup<'_, WallTime>, tiny: &TinyVec<[usize; ELEMENT_COUNT]>) {
	group.bench_function("pop", |b| {
		b.iter_batched(
			|| tiny.clone(),
			|mut tiny| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let value = tiny.pop().expect("Failed to pop");

					black_box(value);
				}

				tiny
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_remove(group: &mut BenchmarkGroup<'_, WallTime>, tiny: &TinyVec<[usize; ELEMENT_COUNT]>) {
	group.bench_function("remove", |b| {
		b.iter_batched(
			|| tiny.clone(),
			|mut tiny| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let len = black_box(0);
					let value = tiny.remove(len);

					black_box(value);
				}

				tiny
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_remove_first(
	group: &mut BenchmarkGroup<'_, WallTime>,
	tiny: &TinyVec<[usize; ELEMENT_COUNT]>,
) {
	group.bench_function("remove_first", |b| {
		b.iter_batched(
			|| tiny.clone(),
			|mut tiny| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let value = tiny.remove(0);

					black_box(value);
				}

				tiny
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_remove_last(
	group: &mut BenchmarkGroup<'_, WallTime>,
	tiny: &TinyVec<[usize; ELEMENT_COUNT]>,
) {
	group.bench_function("remove_last", |b| {
		b.iter_batched(
			|| tiny.clone(),
			|mut tiny| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let len = tiny.len() - 1;
					let value = tiny.remove(len);

					black_box(value);
				}

				tiny
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_swap_remove(
	group: &mut BenchmarkGroup<'_, WallTime>,
	tiny: &TinyVec<[usize; ELEMENT_COUNT]>,
) {
	group.bench_function("swap_remove", |b| {
		b.iter_batched(
			|| tiny.clone(),
			|mut tiny| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let len = black_box(0);
					let value = tiny.swap_remove(len);

					black_box(value);
				}

				tiny
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_swap_remove_last(
	group: &mut BenchmarkGroup<'_, WallTime>,
	tiny: &TinyVec<[usize; ELEMENT_COUNT]>,
) {
	group.bench_function("swap_remove_last", |b| {
		b.iter_batched(
			|| tiny.clone(),
			|mut tiny| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let len = tiny.len() - 1;
					let value = tiny.swap_remove(len);

					black_box(value);
				}

				tiny
			},
			BatchSize::SmallInput,
		);
	});
}

fn benchmark_tiny_vec(c: &mut Criterion) {
	let mut group = c.benchmark_group("TinyVec");
	let elements = u64::try_from(ELEMENT_COUNT).unwrap();

	group.throughput(Throughput::Elements(elements));

	bench_push(&mut group);
	bench_insert(&mut group);
	bench_insert_first(&mut group);
	bench_insert_last(&mut group);

	let mut tiny = TinyVec::<[usize; ELEMENT_COUNT]>::new();

	tiny.extend(0..ELEMENT_COUNT);

	bench_pop(&mut group, &tiny);
	bench_remove(&mut group, &tiny);
	bench_remove_first(&mut group, &tiny);
	bench_remove_last(&mut group, &tiny);
	bench_swap_remove(&mut group, &tiny);
	bench_swap_remove_last(&mut group, &tiny);
}

criterion_group!(benches, benchmark_tiny_vec);
criterion_main!(benches);
