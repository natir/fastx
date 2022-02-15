# A Fast·a·q parser 🧬 💻

[![License](https://img.shields.io/badge/license-MIT-green)](https://github.com/natir/fastx/blob/master/LICENSE)
![CI](https://github.com/natir/fastx/workflows/CI/badge.svg)
[![Documentation](https://github.com/natir/fastx/workflows/Documentation/badge.svg)](https://natir.github.io/fastx/fastx)
[![CodeCov](https://codecov.io/gh/natir/fastx/branch/master/graph/badge.svg)](https://codecov.io/gh/natir/fastx)

We have different fastx parser write in rust, this one was originaly create to be include in [noodles](https://github.com/zaeleus/noodles) but it's not fit in noodles crates target.

## Minimum supported Rust version

Currently the minimum supported Rust version is 1.56.0.

## Evaluate parser

### Criterion

Requirement:
- [cargo criterion](https://github.com/bheisler/cargo-criterion)

```
cargo criterion
```

A html report is generate in `../target/criterion/reports/index.html`

### Hyperfine

Requirement:
- [hyperfine](https://github.com/sharkdp/hyperfine/)
- [seqtk](https://github.com/lh3/seqtk)
- a fastq file

Next script assume FASTQ variable contain path to fastq file.

```
cargo build --release --example fastq2fasta
hyperfine --warmup 3 -n fastx -n seqtk 'target/release/examples/fastq2fasta $FASTQ ' 'seqtk seq -A $FASTQ '
```
