use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;

fn bench_fasta_stats(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-fasta-stats");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fa = manifest.join("tests/golden/tiny.fa");
    c.bench_function("rsomics-fasta-stats golden", |b| {
        b.iter(|| {
            let out = Command::new(black_box(bin))
                .arg(fa.to_str().unwrap())
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_fasta_stats);
criterion_main!(benches);
