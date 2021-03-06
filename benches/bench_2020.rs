use aoc_2019;
use aoc_2020;
use aoc_common::aoc_day::AocDay;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark_2020(c: &mut Criterion) {
    bench::<aoc_2020::Day01>(c, 2020);
    bench::<aoc_2020::Day02>(c, 2020);
    bench::<aoc_2020::Day03>(c, 2020);
    bench::<aoc_2020::Day04>(c, 2020);
    bench::<aoc_2020::Day05>(c, 2020);
    bench::<aoc_2020::Day06>(c, 2020);
    bench::<aoc_2020::Day07>(c, 2020);
    bench::<aoc_2020::Day08>(c, 2020);
    bench::<aoc_2020::Day09>(c, 2020);
    bench::<aoc_2020::Day10>(c, 2020);
    bench::<aoc_2020::Day11>(c, 2020);
    bench::<aoc_2020::Day12>(c, 2020);
    bench::<aoc_2020::Day13>(c, 2020);
    bench::<aoc_2020::Day14>(c, 2020);
    bench::<aoc_2020::Day15>(c, 2020);
    bench::<aoc_2020::Day16>(c, 2020);
    bench::<aoc_2020::Day17>(c, 2020);
    bench::<aoc_2020::Day18>(c, 2020);
    bench::<aoc_2020::Day19>(c, 2020);
    bench::<aoc_2020::Day20>(c, 2020);
    bench::<aoc_2020::Day21>(c, 2020);
    bench::<aoc_2020::Day22>(c, 2020);
    bench::<aoc_2020::Day23>(c, 2020);
    bench::<aoc_2020::Day24>(c, 2020);
}

fn bench<Day: AocDay>(c: &mut Criterion, year: usize) {
    let input = Day::load().unwrap();

    c.bench_function(&format!("{} {} 1", year, Day::day()), |b| {
        b.iter(|| Day::part_1(&input))
    });
    c.bench_function(&format!("{} {} 2", year, Day::day()), |b| {
        b.iter(|| Day::part_2(&input))
    });
}

pub fn criterion_benchmark_2019(c: &mut Criterion) {
    bench::<aoc_2019::day_1::Day01>(c, 2019);
    bench::<aoc_2019::day_02::Day02>(c, 2019);
    bench::<aoc_2019::day_14::Day14>(c, 2019);
    bench::<aoc_2019::day_15::Day15>(c, 2019);
    bench::<aoc_2019::day_16::Day16>(c, 2019);
    bench::<aoc_2019::day_17::Day17>(c, 2019);
    bench::<aoc_2019::day_18::Day18>(c, 2019);
    bench::<aoc_2019::day_19::Day19>(c, 2019);
    bench::<aoc_2019::day_20::Day20>(c, 2019);
    bench::<aoc_2019::day_21::Day21>(c, 2019);
}

criterion_group!(benches, criterion_benchmark_2020, criterion_benchmark_2019);
criterion_main!(benches);
