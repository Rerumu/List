use criterion::{
	black_box, criterion_group, criterion_main, measurement::WallTime, BatchSize, BenchmarkGroup,
	Criterion, Throughput,
};
use list::resizable::Resizable;

const ELEMENT_COUNT: usize = 128;

fn bench_push(group: &mut BenchmarkGroup<'_, WallTime>) {
	group.bench_function("push", |b| {
		b.iter_batched(
			Resizable::<usize, ELEMENT_COUNT>::new,
			|mut resizable| {
				for index in black_box(0..ELEMENT_COUNT) {
					resizable.push(index);
				}

				resizable
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_insert(group: &mut BenchmarkGroup<'_, WallTime>) {
	group.bench_function("insert", |b| {
		b.iter_batched(
			Resizable::<usize, ELEMENT_COUNT>::new,
			|mut resizable| {
				for index in black_box(0..ELEMENT_COUNT) {
					let len = black_box(0);

					resizable.insert(len, index);
				}

				resizable
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_insert_first(group: &mut BenchmarkGroup<'_, WallTime>) {
	group.bench_function("insert_first", |b| {
		b.iter_batched(
			Resizable::<usize, ELEMENT_COUNT>::new,
			|mut resizable| {
				for index in black_box(0..ELEMENT_COUNT) {
					resizable.insert(0, index);
				}

				resizable
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_insert_last(group: &mut BenchmarkGroup<'_, WallTime>) {
	group.bench_function("insert_last", |b| {
		b.iter_batched(
			Resizable::<usize, ELEMENT_COUNT>::new,
			|mut resizable| {
				for index in black_box(0..ELEMENT_COUNT) {
					let len = resizable.len();

					resizable.insert(len, index);
				}

				resizable
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_pop(
	group: &mut BenchmarkGroup<'_, WallTime>,
	resizable: &Resizable<usize, ELEMENT_COUNT>,
) {
	group.bench_function("pop", |b| {
		b.iter_batched(
			|| resizable.clone(),
			|mut resizable| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let value = resizable.pop().expect("Failed to pop");

					black_box(value);
				}

				resizable
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_remove(
	group: &mut BenchmarkGroup<'_, WallTime>,
	resizable: &Resizable<usize, ELEMENT_COUNT>,
) {
	group.bench_function("remove", |b| {
		b.iter_batched(
			|| resizable.clone(),
			|mut resizable| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let len = black_box(0);
					let value = resizable.remove(len);

					black_box(value);
				}

				resizable
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_remove_first(
	group: &mut BenchmarkGroup<'_, WallTime>,
	resizable: &Resizable<usize, ELEMENT_COUNT>,
) {
	group.bench_function("remove_first", |b| {
		b.iter_batched(
			|| resizable.clone(),
			|mut resizable| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let value = resizable.remove(0);

					black_box(value);
				}

				resizable
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_remove_last(
	group: &mut BenchmarkGroup<'_, WallTime>,
	resizable: &Resizable<usize, ELEMENT_COUNT>,
) {
	group.bench_function("remove_last", |b| {
		b.iter_batched(
			|| resizable.clone(),
			|mut resizable| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let len = resizable.len() - 1;
					let value = resizable.remove(len);

					black_box(value);
				}

				resizable
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_swap_remove(
	group: &mut BenchmarkGroup<'_, WallTime>,
	resizable: &Resizable<usize, ELEMENT_COUNT>,
) {
	group.bench_function("swap_remove", |b| {
		b.iter_batched(
			|| resizable.clone(),
			|mut resizable| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let len = black_box(0);
					let value = resizable.swap_remove(len);

					black_box(value);
				}

				resizable
			},
			BatchSize::SmallInput,
		);
	});
}

fn bench_swap_remove_last(
	group: &mut BenchmarkGroup<'_, WallTime>,
	resizable: &Resizable<usize, ELEMENT_COUNT>,
) {
	group.bench_function("swap_remove_last", |b| {
		b.iter_batched(
			|| resizable.clone(),
			|mut resizable| {
				for _ in black_box(0..ELEMENT_COUNT) {
					let len = resizable.len() - 1;
					let value = resizable.swap_remove(len);

					black_box(value);
				}

				resizable
			},
			BatchSize::SmallInput,
		);
	});
}

fn benchmark_resizable(c: &mut Criterion) {
	let mut group = c.benchmark_group("Resizable");
	let elements = u64::try_from(ELEMENT_COUNT).unwrap();

	group.throughput(Throughput::Elements(elements));

	bench_push(&mut group);
	bench_insert(&mut group);
	bench_insert_first(&mut group);
	bench_insert_last(&mut group);

	let mut resizable = Resizable::<usize, ELEMENT_COUNT>::new();

	resizable.extend(0..ELEMENT_COUNT);

	bench_pop(&mut group, &resizable);
	bench_remove(&mut group, &resizable);
	bench_remove_first(&mut group, &resizable);
	bench_remove_last(&mut group, &resizable);
	bench_swap_remove(&mut group, &resizable);
	bench_swap_remove_last(&mut group, &resizable);
}

criterion_group!(benches, benchmark_resizable);
criterion_main!(benches);
