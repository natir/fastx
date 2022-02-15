/* benchmark use */
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fastx;
use rand::seq::SliceRandom;

fn generate_fasta(nb_reads: usize, nb_base: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let nucs = [b'A', b'C', b'T', b'G', b'a', b'c', b't', b'g'];

    let mut writer = fastx::Writer::new(Vec::new());

    for i in 0..nb_reads {
        let mut record = fastx::Record::default();
        record
            .name_mut()
            .extend(format!("{}", i).as_bytes().to_vec());
        *record.description_mut() = Some(format!("random read number {}", i).as_bytes().to_vec());
        record
            .sequence_mut()
            .extend((0..nb_base).map(|_| *nucs.choose(&mut rng).unwrap()));

        writer.write_record(&record).unwrap();
    }

    return writer.get_ref().to_vec();
}

fn generate_fastq(nb_reads: usize, nb_base: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    let nucs = [b'A', b'C', b'T', b'G', b'a', b'c', b't', b'g'];
    let qual: Vec<u8> = (33..126).collect();

    let mut writer = fastx::Writer::new(Vec::new());

    for i in 0..nb_reads {
        let mut record = fastx::Record::default();
        record
            .name_mut()
            .extend(format!("{}", i).as_bytes().to_vec());
        *record.description_mut() = Some(format!("random read number {}", i).as_bytes().to_vec());
        *record.second_description_mut() =
            Some(format!("random read number {}", i).as_bytes().to_vec());
        record
            .sequence_mut()
            .extend((0..nb_base).map(|_| *nucs.choose(&mut rng).unwrap()));
        *record.quality_mut() = Some(
            (0..nb_base)
                .map(|_| *qual.choose(&mut rng).unwrap())
                .collect(),
        );

        writer.write_record(&record).unwrap();
    }

    return writer.get_ref().to_vec();
}

mod short_reads {
    use super::*;

    pub fn fasta(c: &mut Criterion) {
        let mut g = c.benchmark_group("short_reads/fasta");

        let input = generate_fasta(100_000, 150);

        g.bench_function("fastx", |b| {
            b.iter(|| {
                let mut reader = fastx::Reader::new(&input[..]).unwrap();
                for result in reader.records() {
                    let record = result.unwrap();
                    black_box(record);
                }
            })
        });

        g.bench_function("noodles_fasta", |b| {
            b.iter(|| {
                let mut reader = noodles::fasta::Reader::new(&input[..]);
                for result in reader.records() {
                    let record = result.unwrap();
                    black_box(record);
                }
            })
        });
    }

    pub fn fastq(c: &mut Criterion) {
        let mut g = c.benchmark_group("short_reads/fastq");

        let input = generate_fastq(100_000, 150);

        g.bench_function("fastx", |b| {
            b.iter(|| {
                let mut reader = fastx::Reader::new(&input[..]).unwrap();
                for result in reader.records() {
                    let record = result.unwrap();
                    black_box(record);
                }
            })
        });

        g.bench_function("noodles_fastq", |b| {
            b.iter(|| {
                let mut reader = noodles::fastq::Reader::new(&input[..]);
                for result in reader.records() {
                    let record = result.unwrap();
                    black_box(record);
                }
            })
        });
    }
}

mod long_reads {
    use super::*;

    pub fn fasta(c: &mut Criterion) {
        let mut g = c.benchmark_group("long_reads/fasta");

        let input = generate_fasta(10_000, 20_000);

        g.bench_function("fastx", |b| {
            b.iter(|| {
                let mut reader = fastx::Reader::new(&input[..]).unwrap();
                for result in reader.records() {
                    let record = result.unwrap();
                    black_box(record);
                }
            })
        });

        g.bench_function("fasta", |b| {
            b.iter(|| {
                let mut reader = noodles::fasta::Reader::new(&input[..]);
                for result in reader.records() {
                    let record = result.unwrap();
                    black_box(record);
                }
            })
        });
    }

    pub fn fastq(c: &mut Criterion) {
        let mut g = c.benchmark_group("long_reads/fastq");

        let input = generate_fastq(10_000, 20_000);

        g.bench_function("fastx", |b| {
            b.iter(|| {
                let mut reader = fastx::Reader::new(&input[..]).unwrap();
                for result in reader.records() {
                    let record = result.unwrap();
                    black_box(record);
                }
            })
        });

        g.bench_function("fastq", |b| {
            b.iter(|| {
                let mut reader = noodles::fastq::Reader::new(&input[..]);
                for result in reader.records() {
                    let record = result.unwrap();
                    black_box(record);
                }
            })
        });
    }
}

fn setup(c: &mut Criterion) {
    short_reads::fasta(c);
    short_reads::fastq(c);

    long_reads::fasta(c);
    long_reads::fastq(c);
}

criterion_group!(benches, setup);

criterion_main!(benches);
